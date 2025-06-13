use super::core::ReportExt;
use crate::entity::{DateSelection, Event, NextPageResults};
use crate::env::query::FindEvents;
use chrono::Utc;

#[derive(Debug, Clone, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SearchEvents {
    #[builder(into, default = 20u16)]
    pub limit: u16,

    #[builder(into)]
    pub offset: Option<u16>,

    #[builder(into, default = DateSelection::After(Utc::now()))]
    pub date_selection: DateSelection,
}

#[derive(Debug, thiserror::Error)]
pub enum SearchEventsError {
    #[error("exceeded max limit of {0}")]
    ExceededLimitMax(u16),

    #[error(transparent)]
    Database(sqlx::Error),
}

impl ReportExt for SearchEvents {
    type Success = NextPageResults<Event>;
    type Failure = SearchEventsError;

    async fn exec(
        &self,
        ctx: &crate::Context,
        env: &impl crate::env::Env,
    ) -> Result<Self::Success, Self::Failure> {
        const MAX_ANON_RESULTS: u16 = 50;
        if ctx.is_nobody() && self.limit > MAX_ANON_RESULTS {
            return Err(SearchEventsError::ExceededLimitMax(MAX_ANON_RESULTS));
        }

        let mut data = env
            .exec_query(&FindEvents {
                limit: self.limit + 1,
                offset: self.offset,
                date_selection: self.date_selection,
            })
            .await
            .map_err(SearchEventsError::Database)?;

        let has_next_page = data.len() > self.limit as usize;
        if has_next_page {
            data.pop();
        }
        Ok(NextPageResults {
            data,
            has_next_page,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::{date::today, env::*, prelude::*};
    use crate::{Context, entity::UserId};
    type Result<T> = std::result::Result<T, SearchEventsError>;

    fn event_list(size: usize) -> Vec<Event> {
        std::iter::repeat_n(Event::default(), size).collect()
    }

    #[rstest]
    #[case(0, 0, false)]
    #[case(1, 1, false)]
    #[case(11, 10, true)]
    #[tokio::test]
    async fn returning_results(
        #[case] list_length: usize,
        #[case] expected_count: usize,
        #[case] next_page: bool,
        mut mock_env: MockEnvironment,
    ) -> Result<()> {
        let date_selection = DateSelection::After(today());
        let ctx = Context::User(UserId(42));
        let search = SearchEvents::builder()
            .limit(10u16)
            .offset(20u16)
            .date_selection(date_selection)
            .build();

        mock_env
            .expect_exec_query()
            .withf(move |find: &FindEvents| {
                assert_eq!(find.limit, 11);
                assert_eq!(find.offset, Some(20));
                assert_eq!(find.date_selection, date_selection);
                true
            })
            .returning(move |_| Ok(event_list(list_length)));

        let results = search.exec(&ctx, &mock_env).await?;
        assert_eq!(results.has_next_page, next_page);
        assert_eq!(results.data.len(), expected_count);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn limit_anon_result_size(mock_env: MockEnvironment) -> Result<()> {
        let search = SearchEvents::builder().limit(100u16).build();
        let ctx = Context::Nobody;
        let error = search
            .exec(&ctx, &mock_env)
            .await
            .expect_err("exceeded max error");
        assert!(matches!(error, SearchEventsError::ExceededLimitMax(50)));
        Ok(())
    }
}
