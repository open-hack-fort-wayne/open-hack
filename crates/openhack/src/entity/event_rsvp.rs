use super::{EventId, RsvpStatus, UserId};
use crate::common::DateTimeUtc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "runtime", derive(sqlx::FromRow))]
pub struct EventRsvp {
    #[builder(into)]
    pub event_id: EventId,

    #[builder(into)]
    pub user_id: UserId,

    #[builder(into)]
    pub status: RsvpStatus,

    #[builder(into)]
    pub extra_attendee_count: i32,

    #[builder(into)]
    pub created_at: DateTimeUtc,

    #[builder(into)]
    pub updated_at: DateTimeUtc,
}
