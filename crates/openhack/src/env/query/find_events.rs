use super::*;
use crate::common::DateTimeUtc;
use crate::entity::{DateSelection, Event};

#[derive(DebugMore, Clone, Serialize, Deserialize, bon::Builder)]
pub struct FindEvents {
    #[builder(into)]
    pub limit: u16,

    #[builder(into)]
    pub offset: Option<u16>,

    #[builder(into)]
    pub date_selection: DateSelection,
}

impl Query for FindEvents {
    type Success = Vec<Event>;
    type Failure = sqlx::Error;

    async fn exec(&self, conn: &sqlx::PgPool) -> Result<Self::Success, Self::Failure> {
        let mut events_query = sqlx::QueryBuilder::new(
            r#"
            SELECT
                id,
                creator_id,
                scheduled_date,
                duration_in_mins,
                name,
                location,
                details,
                created_at,
                updated_at
            FROM events
            WHERE 
            "#,
        );

        match self.date_selection {
            DateSelection::After(date) => {
                events_query.push("scheduled_date > ");
                events_query.push_bind(date);
            }
            DateSelection::Before(date) => {
                events_query.push("scheduled_date < ");
                events_query.push_bind(date);
            }
            DateSelection::Between(left, right) => {
                events_query.push("scheduled_date BETWEEN ");
                events_query.push_bind(left);
                events_query.push(" AND ");
                events_query.push_bind(right);
            }
        }

        events_query.push(" ORDER BY scheduled_date ASC");
        events_query.push(" LIMIT ");
        events_query.push_bind(self.limit as i32);

        if let Some(offset) = self.offset {
            events_query.push(" OFFSET ");
            events_query.push_bind(offset as i32);
        }

        let events = events_query
            .build_query_as::<Event>()
            .fetch_all(conn)
            .await?;

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Duration;
    use crate::support::date::*;

    fn events(date_select: DateSelection) -> FindEvents {
        FindEvents::builder()
            .limit(10u16)
            .offset(0u16)
            .date_selection(date_select)
            .build()
    }

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn finds_rows(pool: PgPool) -> Result<(), sqlx::Error> {
        let has_row = async |search: FindEvents| {
            let events = search.exec(&pool).await?;
            assert_eq!(events.len(), 1);
            Ok::<(), sqlx::Error>(())
        };

        has_row(events(DateSelection::After(today()))).await?;
        has_row(events(DateSelection::Before(two_days_from_now()))).await?;
        has_row(events(DateSelection::Between(today(), two_days_from_now()))).await?;
        Ok(())
    }

    #[sqlx::test(fixtures("./fixtures/jdongs-event.sql"))]
    async fn excludes_rows(pool: PgPool) -> Result<(), sqlx::Error> {
        let no_row = async |search: FindEvents| {
            let events = search.exec(&pool).await?;
            assert!(events.is_empty());
            Ok::<(), sqlx::Error>(())
        };

        no_row(events(DateSelection::After(two_days_from_now()))).await?;
        no_row(events(DateSelection::Before(today()))).await?;
        no_row(events(DateSelection::Between(yesterday(), today()))).await?;
        Ok(())
    }
}
