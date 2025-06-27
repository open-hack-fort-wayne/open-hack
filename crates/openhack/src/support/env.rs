use crate::env::Env;
use crate::env::resource::database::{Database, Query};
use crate::env::resource::password_manager::PasswordManager;

#[rstest::fixture]
pub fn mock_env() -> MockEnvironment {
    MockEnvironment::new()
}

mockall::mock! {
    pub Environment {}

    impl Database for Environment {
        fn get_conn(&self) -> &sqlx::PgPool {
            todo!()
        }

        async fn exec_query<Q>(&self, query: &Q) -> Result<Q::Success, Q::Failure>
        where
            Q: Query
        {
            todo!()
        }
    }

    impl PasswordManager for Environment {
        fn get_password_hasher(&self) -> &openhack_auth::PasswordHasher {
            todo!()
        }

        fn verify_password(&self, password: &str, hash: &str) -> Result<(), openhack_auth::HashError> {
            todo!()
        }

        fn hash_password(&self, password: &str) -> Result<String, openhack_auth::HashError> {
            todo!()
        }
    }

    impl Env for Environment {}
}
