use super::*;

/// # Database
///
/// Simple adapter trait around [sqlx] which works in concert with
/// the [Query] trait to provide an isolated abstraction around its
/// usage.
///
pub trait Database {
    /// # Get Connection
    ///
    /// This should be the only method you need to actually implement
    /// to have access to the database.  The remainder of the methods
    /// in this trait rely on strategy pattern traits.
    ///
    fn get_conn(&self) -> &sqlx::PgPool;

    /// # Execute Query
    ///
    /// Default query implementation which provides the [Query] trait
    /// access to the connection for execution.  The query given is
    /// traced with instrumentation for debugging and monitoring.
    ///
    #[tracing::instrument(skip(self))]
    fn exec_query<Q>(&self, query: &Q) -> impl Future<Output = Result<Q::Success, Q::Failure>>
    where
        Q: Query,
    {
        query.exec(self.get_conn())
    }
}

// Blanket Database impl for any resource of pgPool
impl<T> Database for T
where
    T: Resource<sqlx::PgPool>,
{
    fn get_conn(&self) -> &sqlx::PgPool {
        self.as_res()
    }
}

/// # Query
///
/// Strategy trait used by any [Database] which can provide the required
/// [Connection] for execution.  This is meant to serve as an abstraction
/// around the fetching and marshaling of data from the database.
///
/// **WARNING**:  The query is traced by the default database.  Please
/// make sure any sensitive data is omitted by the [Debug] output.
///
pub trait Query: Stable + 'static {
    type Success;
    type Failure;

    /// run query and return result
    fn exec(
        &self,
        _conn: &sqlx::PgPool,
    ) -> impl Future<Output = Result<Self::Success, Self::Failure>> {
        async { todo!() }
    }
}

#[cfg(test)]
mockall::mock! {
    Db {}

    impl Database for Db {

        fn get_conn(&self) -> &sqlx::PgPool {
            unreachable!("Pool not available")
        }

        fn exec_query<Q>(&self, query: &Q) -> impl Future<Output = Result<Q::Success, Q::Failure>>
        where
            Q: Query,
        {
            query.exec(self.get_conn())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockall::predicate::*;

    #[derive(Debug, PartialEq, Eq)]
    struct SampleQuery(i64);
    impl Query for SampleQuery {
        type Success = i32;
        type Failure = anyhow::Error;
    }

    #[tokio::test]
    async fn the_mock_works() -> anyhow::Result<()> {
        let mut db = MockDb::new();
        db.expect_exec_query()
            .with(eq(&SampleQuery(42)))
            .times(1)
            .returning(|_: &SampleQuery| Box::pin(async { Ok(13) }));
        let val = db.exec_query(&SampleQuery(42)).await?;
        assert_eq!(val, 13);
        Ok(())
    }
}
