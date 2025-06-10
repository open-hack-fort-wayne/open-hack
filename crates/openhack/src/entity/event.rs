use super::{EventId, UserId};
use crate::common::DateTimeUtc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "runtime", derive(sqlx::FromRow))]
pub struct Event {
    #[builder(into)]
    pub id: EventId,

    #[builder(into)]
    pub creator_id: UserId,

    #[builder(into)]
    pub scheduled_date: DateTimeUtc,

    #[builder(into)]
    pub duration_in_mins: i32,

    #[builder(into)]
    pub name: String,

    #[builder(into)]
    pub location: String,

    #[builder(into)]
    pub details: String,

    #[builder(into)]
    pub created_at: DateTimeUtc,

    #[builder(into)]
    pub updated_at: DateTimeUtc,
}

mod impls {
    use super::*;
    use crate::common::Duration;
    use ::validator::ValidationError;

    impl Event {
        pub(crate) fn validate_future_scheduled_date(
            date: &DateTimeUtc,
        ) -> Result<(), ValidationError> {
            if *date < chrono::Utc::now() {
                return Err(ValidationError::new("scheduled_date_not_in_future"));
            }

            Ok(())
        }

        pub(crate) fn validate_duration_long_enough(
            duration: &Duration,
        ) -> Result<(), ValidationError> {
            if duration.num_minutes() < 30 {
                return Err(ValidationError::new("duration_too_short"));
            }
            Ok(())
        }
    }
}
