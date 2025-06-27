use super::core::CommandExt;
use crate::common::{DateTimeUtc, Duration};
use crate::entity::Event;
use crate::env::query::InsertEvent;
use ::validator::Validate;

#[derive(derive_more::Debug, Clone, Validate, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvent {
    #[builder(into)]
    #[validate(custom(function = "Event::validate_future_scheduled_date"))]
    pub scheduled_date: DateTimeUtc,

    #[builder(into)]
    #[validate(custom(function = "Event::validate_duration_long_enough"))]
    pub duration: Duration,

    #[builder(into)]
    #[validate(length(min = 7))]
    pub name: String,

    #[builder(into)]
    #[validate(length(min = 7))]
    pub location: String,

    #[builder(into)]
    #[validate(length(min = 20))]
    pub details: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateEventError {
    #[error(transparent)]
    Validations(#[from] validator::ValidationErrors),

    #[error("user required")]
    UserRequired,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl CommandExt for CreateEvent {
    type Success = Event;
    type Failure = CreateEventError;

    async fn exec(
        &self,
        ctx: &crate::Context,
        env: &impl crate::env::Env,
    ) -> Result<Self::Success, Self::Failure> {
        let creator_id = ctx.user_id().ok_or(CreateEventError::UserRequired)?;
        self.validate()?;
        env.exec_query(
            &InsertEvent::builder()
                .creator_id(creator_id)
                .scheduled_date(self.scheduled_date)
                .duration_in_mins(self.duration.num_minutes() as i32)
                .name(self.name.as_str())
                .location(self.location.as_str())
                .details(self.details.as_str())
                .build(),
        )
        .await
        .map_err(CreateEventError::Database)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::entity::{EventId, UserId};
    use crate::support::{env::*, prelude::*};
    use ::chrono::Utc;
    type Result<T> = std::result::Result<T, CreateEventError>;

    #[fixture]
    fn create_event() -> CreateEvent {
        CreateEvent::builder()
            .scheduled_date(Utc::now() + Duration::days(1))
            .duration(Duration::minutes(90))
            .name("Big Show")
            .location("Big Top")
            .details("Welcome to the Big Show!")
            .build()
    }

    fn big_show() -> Event {
        Event {
            id: EventId(42),
            ..Default::default()
        }
    }

    #[rstest]
    #[tokio::test]
    async fn happy_path(mut mock_env: MockEnvironment, create_event: CreateEvent) -> Result<()> {
        let expected_mins = create_event.duration.num_minutes() as i32;
        mock_env
            .expect_exec_query()
            .withf(move |x: &InsertEvent| {
                assert_eq!(x.creator_id, UserId(1));
                assert_eq!(x.duration_in_mins, expected_mins);
                assert_eq!(x.name, "Big Show");
                assert_eq!(x.location, "Big Top");
                assert_eq!(x.details, "Welcome to the Big Show!");
                true
            })
            .returning(|_| Ok(big_show()));

        let ctx = Context::User(UserId(1));
        let event = create_event.exec(&ctx, &mock_env).await?;
        assert_eq!(event.id, EventId(42));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn nobody_errors(mock_env: MockEnvironment, create_event: CreateEvent) -> Result<()> {
        let nobody = Context::Nobody;
        let error = create_event
            .exec(&nobody, &mock_env)
            .await
            .expect_err("needs user");
        assert!(matches!(error, CreateEventError::UserRequired));
        Ok(())
    }
}
