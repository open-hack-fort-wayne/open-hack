use super::*;

/// currently a marker to determine a database connection
pub trait Connection: Stable {}

impl Connection for sqlx::PgPool {}

/// # Database
///
/// Simple adapter trait around [sqlx] which works in concert with
/// the [Query] trait to provide an isolated abstraction around its
/// usage.
///
pub trait Database {
    /// Remote access to the data
    type Conn: Connection;

    /// # Get Connection
    ///
    /// This should be the only method you need to actually implement
    /// to have access to the database.  The remainder of the methods
    /// in this trait rely on strategy pattern traits.
    ///
    fn get_conn(&self) -> &Self::Conn;

    /// # Execute Query
    ///
    /// Default query implementation which provides the [Query] trait
    /// access to the connection for execution.  The query given is
    /// traced with instrumentation for debugging and monitoring.
    ///
    #[instrument(skip(self))]
    fn exec_query<Q>(&self, query: &Q) -> impl Future<Output = Result<Q::Output>>
    where
        Q: Query<<Self as Database>::Conn>,
    {
        query.exec(self.get_conn())
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
pub trait Query<Connection>: Stable + 'static {
    type Output;

    /// run query and return result
    fn exec(&self, conn: &Connection) -> impl Future<Output = Result<Self::Output>> {
        async move { anyhow::bail!("Query Not Implemented {:?}", self) }
    }
}

#[cfg(test)]
mockall::mock! {
    Db {}

    impl Database for Db {
        type Conn = sqlx::PgPool;

        fn get_conn(&self) -> &sqlx::PgPool {
            unreachable!("Pool not available")
        }

        fn exec_query<Q>(&self, query: &Q) -> impl Future<Output = Result<Q::Output>>
        where
            Q: Query<sqlx::PgPool>,
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
    impl Query<sqlx::PgPool> for SampleQuery {
        type Output = i32;
    }

    #[tokio::test]
    async fn the_mock_works() -> Result<()> {
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
