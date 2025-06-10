use super::*;
use crate::{
    common::DateTimeUtc,
    entity::{EventId, EventRsvp, RsvpStatus, UserId},
};
use sqlx::Row;

#[derive(DebugMore, Clone, Serialize, Deserialize, bon::Builder)]
pub struct UpsertEventRsvp {
    #[builder(into)]
    pub event_id: EventId,

    #[builder(into)]
    pub user_id: UserId,

    #[builder(into, default = 0)]
    pub extra_attendee_count: u8,

    #[builder(into)]
    pub status: RsvpStatus,
}

impl Query for UpsertEventRsvp {
    type Success = EventRsvp;
    type Failure = sqlx::Error;

    async fn exec(&self, conn: &sqlx::PgPool) -> Result<Self::Success, Self::Failure> {
        let row = sqlx::query(
            r#"
                INSERT INTO event_rsvps (
                    event_id, user_id, extra_attendee_count, status
                )
                VALUES (
                    $1, $2, $3, $4
                )
                ON CONFLICT (event_id, user_id) DO UPDATE
                    SET extra_attendee_count = EXCLUDED.extra_attendee_count,
                        status = EXCLUDED.status,
                        updated_at = CURRENT_TIMESTAMP
                RETURNING
                    event_id,
                    user_id,
                    updated_at as "updated_at!",
                    created_at as "created_at!",
                    status as "status: RsvpStatus",
                    extra_attendee_count
            "#,
        )
        .bind(self.event_id)
        .bind(self.user_id)
        .bind(self.extra_attendee_count as i32)
        .bind(self.status)
        .fetch_one(conn)
        .await?;

        Ok(EventRsvp {
            event_id: row.try_get("event_id")?,
            user_id: row.try_get("user_id")?,
            status: row.try_get("status: RsvpStatus")?,
            extra_attendee_count: row.try_get("extra_attendee_count")?,
            created_at: row.try_get("created_at!")?,
            updated_at: row.try_get("updated_at!")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn updates_rsvp(pool: PgPool) -> Result<(), sqlx::Error> {
        let upsert = UpsertEventRsvp::builder()
            .user_id(7)
            .event_id(73)
            .extra_attendee_count(4)
            .status(RsvpStatus::Yes)
            .build();

        let rsvp = upsert.exec(&pool).await?;
        assert!(rsvp.updated_at > rsvp.created_at);
        assert_eq!(rsvp.extra_attendee_count, 4);
        assert_eq!(rsvp.status, RsvpStatus::Yes);
        Ok(())
    }

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn creates_rsvp(pool: PgPool) -> Result<(), sqlx::Error> {
        let upsert = UpsertEventRsvp::builder()
            .user_id(42)
            .event_id(73)
            .extra_attendee_count(0)
            .status(RsvpStatus::No)
            .build();
        let rsvp = upsert.exec(&pool).await?;
        assert_eq!(rsvp.updated_at, rsvp.created_at);
        assert_eq!(rsvp.user_id, UserId(42));
        assert_eq!(rsvp.event_id, EventId(73));
        assert_eq!(rsvp.status, RsvpStatus::No);
        Ok(())
    }
}
