use super::*;
use crate::common::{DateTimeUtc, Duration};
use crate::entity::{Event, EventId, UserId};
use ::sqlx::FromRow;
use ::validator::{Validate, ValidationError};

#[derive(DebugMore, Clone, Serialize, Deserialize, bon::Builder, Validate)]
pub struct UpdateEventForCreator {
    #[builder(into)]
    pub event_id: EventId,

    #[builder(into)]
    pub creator_id: UserId,

    #[builder(into)]
    pub scheduled_date: Option<DateTimeUtc>,

    #[builder(into)]
    pub duration: Option<Duration>,

    #[builder(into)]
    pub name: Option<String>,

    #[builder(into)]
    pub location: Option<String>,

    #[builder(into)]
    pub details: Option<String>,
}

impl Query for UpdateEventForCreator {
    type Success = Option<Event>;
    type Failure = sqlx::Error;

    async fn exec(&self, conn: &sqlx::PgPool) -> Result<Self::Success, Self::Failure> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE events SET ");
        let mut pairs = builder.separated(", ");

        if let Some(date) = self.scheduled_date {
            pairs.push("scheduled_date = ");
            pairs.push_bind_unseparated(date);
        }

        if let Some(time) = self.duration {
            pairs.push("duration_in_mins = ");
            pairs.push_bind_unseparated(time.num_minutes() as i32);
        }

        if let Some(name) = self.name.as_ref() {
            pairs.push("name = ");
            pairs.push_bind_unseparated(name);
        }

        if let Some(location) = self.location.as_ref() {
            pairs.push("location = ");
            pairs.push_bind_unseparated(location);
        }

        if let Some(details) = self.details.as_ref() {
            pairs.push("details = ");
            pairs.push_bind_unseparated(details);
        }

        pairs.push("updated_at = CURRENT_TIMESTAMP ");

        builder.push("WHERE id = ");
        builder.push_bind(self.event_id);
        builder.push(" AND creator_id = ");
        builder.push_bind(self.creator_id);

        builder.push(
            r#"
            RETURNING
                id,
                creator_id,
                scheduled_date,
                duration_in_mins,
                name,
                location,
                details,
                created_at,
                updated_at
            "#,
        );

        Ok(match builder.build().fetch_optional(conn).await? {
            Some(row) => Some(Event::from_row(&row)?),
            None => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn touch_record(pool: PgPool) -> Result<(), sqlx::Error> {
        let update = UpdateEventForCreator::builder()
            .event_id(73)
            .creator_id(42)
            .build();
        let event = update.exec(&pool).await?.unwrap();
        assert_eq!(event.id, EventId(73));
        assert_eq!(event.creator_id, UserId(42));
        assert!(event.updated_at > event.created_at);
        assert_eq!(event.name, "Jdongs Big Day");
        assert_eq!(event.location, "Jdongs House");
        assert_eq!(event.details, "This one is for Jdong!");
        assert_eq!(event.duration_in_mins, 90);
        Ok(())
    }

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn wrong_creator_id_is_none(pool: PgPool) -> Result<(), sqlx::Error> {
        let update = UpdateEventForCreator::builder()
            .event_id(73)
            .creator_id(7)
            .build();
        let missing_data = update.exec(&pool).await?;
        assert!(missing_data.is_none());
        Ok(())
    }

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn update_everything(pool: PgPool) -> Result<(), sqlx::Error> {
        let event = UpdateEvent::builder()
            .event_id(73)
            .creator_id(42)
            .duration(Duration::minutes(45))
            .name("LTrain Party")
            .location("LTrains Barn")
            .details("No party like a LTrain party")
            .build()
            .exec(&pool)
            .await?
            .unwrap();
        assert_eq!(event.id, EventId(73));
        assert_eq!(event.creator_id, UserId(42));
        assert!(event.updated_at > event.created_at);
        assert_eq!(event.name, "LTrain Party");
        assert_eq!(event.location, "LTrains Barn");
        assert_eq!(event.details, "No party like a LTrain party");
        assert_eq!(event.duration_in_mins, 45);
        Ok(())
    }
}
