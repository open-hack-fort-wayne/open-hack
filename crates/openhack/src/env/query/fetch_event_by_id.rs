use super::*;
use crate::entity::{Event, EventId};

#[derive(DebugMore, Clone, Serialize, Deserialize)]
pub struct FetchEventById(pub EventId);

impl Query for FetchEventById {
    type Success = Option<Event>;
    type Failure = sqlx::Error;

    async fn exec(&self, conn: &sqlx::PgPool) -> Result<Self::Success, Self::Failure> {
        sqlx::query_as!(
            Event,
            r#"
                SELECT
                    id,
                    creator_id,
                    scheduled_date as "scheduled_date!",
                    duration_in_mins,
                    name,
                    location,
                    details,
                    created_at as "created_at!",
                    updated_at as "updated_at!"
                FROM events
                WHERE id = $1
            "#,
            self.0.0 as i32
        )
        .fetch_optional(conn)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn fetches_expected_result(pool: PgPool) -> Result<(), sqlx::Error> {
        let event = FetchEventById(EventId(73)).exec(&pool).await?.unwrap();
        assert_eq!(event.name, "Jdongs Big Day");
        Ok(())
    }
}
