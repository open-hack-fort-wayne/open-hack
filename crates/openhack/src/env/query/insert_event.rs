use super::*;
use crate::common::DateTimeUtc;
use crate::entity::{Event, UserId};

#[derive(DebugMore, Clone, Serialize, Deserialize, bon::Builder)]
pub struct InsertEvent {
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
}

impl Query for InsertEvent {
    type Success = Event;
    type Failure = sqlx::Error;

    async fn exec(&self, conn: &sqlx::PgPool) -> Result<Self::Success, Self::Failure> {
        sqlx::query_as!(
            Event,
            r#"
                INSERT INTO events (
                    creator_id, scheduled_date, duration_in_mins,
                    name, location, details
                )
                VALUES(
                    $1, $2, $3,
                    $4, $5, $6
                )
                RETURNING
                    id,
                    creator_id,
                    scheduled_date as "scheduled_date!",
                    duration_in_mins,
                    name,
                    location,
                    details,
                    created_at as "created_at!",
                    updated_at as "updated_at!"
            "#,
            self.creator_id.0 as i32,
            self.scheduled_date,
            self.duration_in_mins,
            self.name.as_str(),
            self.location.as_str(),
            self.details.as_str(),
        )
        .fetch_one(conn)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::chrono::{Duration, Utc};

    #[sqlx::test(fixtures("./fixtures/john-smith.sql"))]
    async fn works_as_expected(pool: PgPool) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let insert = InsertEvent::builder()
            .creator_id(1)
            .scheduled_date(now)
            .duration_in_mins(90)
            .name("Big Show")
            .location("Big Top")
            .details("come one, come all")
            .build();

        let event = insert.exec(&pool).await?;
        assert_eq!(event.creator_id, UserId(1));
        // psql looses a little resolution
        assert!((event.scheduled_date - now).abs() <= Duration::seconds(1));
        assert_eq!(event.duration_in_mins, 90);
        assert_eq!(event.name, "Big Show");
        assert_eq!(event.location, "Big Top");
        assert_eq!(event.details, "come one, come all");
        Ok(())
    }
}
