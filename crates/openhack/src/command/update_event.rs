use super::core::CommandExt;
use crate::common::{DateTimeUtc, Duration};
use crate::entity::{Event, EventId};
use crate::env::query::UpdateEventForCreator;
use ::validator::Validate;

#[derive(derive_more::Debug, Clone, Validate, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateEvent {
    #[builder(into)]
    pub event_id: EventId,

    #[builder(into)]
    #[validate(custom(function = "Event::validate_future_scheduled_date"))]
    pub scheduled_date: Option<DateTimeUtc>,

    #[builder(into)]
    #[validate(custom(function = "Event::validate_duration_long_enough"))]
    pub duration: Option<Duration>,

    #[builder(into)]
    #[validate(length(min = 7))]
    pub name: Option<String>,

    #[builder(into)]
    #[validate(length(min = 7))]
    pub location: Option<String>,

    #[builder(into)]
    #[validate(length(min = 20))]
    pub details: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateEventError {
    #[error(transparent)]
    Validations(#[from] validator::ValidationErrors),

    #[error("could not update event {0:?}")]
    MissingEvent(EventId),

    #[error("user required")]
    UserRequired,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl CommandExt for UpdateEvent {
    type Success = Event;
    type Failure = UpdateEventError;

    async fn exec(
        &self,
        ctx: &crate::Context,
        env: &impl crate::env::Env,
    ) -> Result<Self::Success, Self::Failure> {
        let creator_id = ctx.user_id().ok_or(UpdateEventError::UserRequired)?;
        self.validate()?;
        env.exec_query(&UpdateEventForCreator {
            event_id: self.event_id,
            duration: self.duration,
            scheduled_date: self.scheduled_date,
            name: self.name.clone(),
            location: self.location.clone(),
            details: self.details.clone(),
            creator_id,
        })
        .await
        .map_err(UpdateEventError::Database)?
        .ok_or_else(|| UpdateEventError::MissingEvent(self.event_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::entity::{EventId, UserId};
    use crate::support::{env::*, prelude::*};
    use ::chrono::Utc;
    type Result<T> = std::result::Result<T, UpdateEventError>;

    #[fixture]
    fn update_event() -> UpdateEvent {
        UpdateEvent::builder()
            .event_id(18)
            .scheduled_date(Utc::now() + Duration::days(1))
            .duration(Duration::minutes(90))
            .name("Big Show")
            .location("Big Top")
            .details("Welcome to the Big Show!")
            .build()
    }

    fn big_show() -> Event {
        Event {
            id: EventId(18),
            ..Default::default()
        }
    }

    #[rstest]
    #[tokio::test]
    async fn happy_path(mut mock_env: MockEnvironment, update_event: UpdateEvent) -> Result<()> {
        let expected_date = update_event.scheduled_date.unwrap();
        mock_env
            .expect_exec_query()
            .withf(move |x: &UpdateEventForCreator| {
                assert_eq!(x.event_id, EventId(18));
                assert_eq!(x.creator_id, UserId(42));
                assert_eq!(x.duration.unwrap().num_minutes(), 90);
                assert_eq!(x.name.clone().unwrap(), "Big Show");
                assert_eq!(x.location.clone().unwrap(), "Big Top");
                assert_eq!(x.details.clone().unwrap(), "Welcome to the Big Show!");
                assert_eq!(x.scheduled_date.unwrap(), expected_date);
                true
            })
            .returning(|_| Ok(Some(big_show())));
        let ctx = Context::User(UserId(42));
        let event = update_event.exec(&ctx, &mock_env).await?;
        assert_eq!(event.id, EventId(18));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn missing_event(mut mock_env: MockEnvironment, update_event: UpdateEvent) -> Result<()> {
        mock_env
            .expect_exec_query()
            .withf(move |x: &UpdateEventForCreator| {
                assert_eq!(x.event_id, EventId(18));
                assert_eq!(x.creator_id, UserId(42));
                true
            })
            .returning(|_| Ok(None));
        let ctx = Context::User(UserId(42));
        let error = update_event
            .exec(&ctx, &mock_env)
            .await
            .expect_err("missing event");
        assert!(matches!(error, UpdateEventError::MissingEvent(EventId(18))));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn missing_user(mock_env: MockEnvironment, update_event: UpdateEvent) -> Result<()> {
        let ctx = Context::Nobody;
        let error = update_event
            .exec(&ctx, &mock_env)
            .await
            .expect_err("missing user");
        assert!(matches!(error, UpdateEventError::UserRequired));
        Ok(())
    }
}
