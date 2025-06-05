use derive_more::Debug;
use resource::Resource;
use resource::database::Database;
use resource::password_manager::PasswordManager;

pub(crate) mod query;
pub(crate) mod resource;

/// # Openhack Environment Trait
///
/// This trait is your access to global resources such
/// databases, http clients, etc.
///
pub trait Env: Database + PasswordManager {}

/// # Openhack Live Environment
#[derive(Debug, bon::Builder)]
pub struct Environment {
    #[debug(skip)]
    database: sqlx::PgPool,

    #[debug(skip)]
    password_hasher: openhack_auth::PasswordHasher,
}

impl Resource<sqlx::PgPool> for Environment {
    fn as_res(&self) -> &sqlx::PgPool {
        &self.database
    }
}

impl Resource<openhack_auth::PasswordHasher> for Environment {
    fn as_res(&self) -> &openhack_auth::PasswordHasher {
        &self.password_hasher
    }
}

impl Env for Environment {}
