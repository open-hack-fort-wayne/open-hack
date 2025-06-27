use super::core::CommandExt;
use crate::entity::{EventId, EventRsvp, RsvpStatus};
use crate::env::query::UpsertEventRsvp;
use ::validator::Validate;

#[derive(derive_more::Debug, Clone, Validate, bon::Builder)]
pub struct UpsertRsvp {
    #[builder(into)]
    pub event_id: EventId,

    #[builder(into, default = RsvpStatus::Maybe)]
    pub status: RsvpStatus,

    #[builder(into, default = 0)]
    pub extra_attendee_count: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum UpsertRsvpError {
    #[error(transparent)]
    Validations(#[from] validator::ValidationErrors),

    #[error(transparent)]
    Database(sqlx::Error),

    #[error("user required")]
    UserRequired,
}

impl CommandExt for UpsertRsvp {
    type Success = EventRsvp;
    type Failure = UpsertRsvpError;

    async fn exec(
        &self,
        ctx: &crate::Context,
        env: &impl crate::env::Env,
    ) -> Result<Self::Success, Self::Failure> {
        let user_id = ctx.user_id().ok_or(UpsertRsvpError::UserRequired)?;
        self.validate()?;
        env.exec_query(&UpsertEventRsvp {
            event_id: self.event_id,
            status: self.status,
            extra_attendee_count: self.extra_attendee_count,
            user_id,
        })
        .await
        .map_err(UpsertRsvpError::Database)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::context::Context;
    use crate::entity::UserId;
    use crate::support::{env::*, prelude::*};
    type Result<T> = std::result::Result<T, UpsertRsvpError>;

    #[fixture]
    fn upsert_rsvp() -> UpsertRsvp {
        UpsertRsvp::builder()
            .event_id(99)
            .status(RsvpStatus::No)
            .extra_attendee_count(7)
            .build()
    }

    fn rsvp() -> EventRsvp {
        let now = Utc::now();
        EventRsvp::builder()
            .event_id(66)
            .user_id(99)
            .extra_attendee_count(5)
            .status(RsvpStatus::Maybe)
            .created_at(now)
            .updated_at(now)
            .build()
    }

    #[rstest]
    #[tokio::test]
    async fn happy_path(mut mock_env: MockEnvironment, upsert_rsvp: UpsertRsvp) -> Result<()> {
        mock_env
            .expect_exec_query()
            .withf(|x: &UpsertEventRsvp| {
                assert_eq!(x.event_id, EventId(99));
                assert_eq!(x.user_id, UserId(22));
                assert_eq!(x.extra_attendee_count, 7);
                assert_eq!(x.status, RsvpStatus::No);
                true
            })
            .returning(|_| Ok(rsvp()));
        let ctx = Context::User(UserId(22));
        let event = upsert_rsvp.exec(&ctx, &mock_env).await?;
        assert_eq!(event.event_id, EventId(66));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn missing_user(mock_env: MockEnvironment, upsert_rsvp: UpsertRsvp) -> Result<()> {
        let ctx = Context::Nobody;
        let error = upsert_rsvp
            .exec(&ctx, &mock_env)
            .await
            .expect_err("missing user");
        assert!(matches!(error, UpsertRsvpError::UserRequired));
        Ok(())
    }
}
