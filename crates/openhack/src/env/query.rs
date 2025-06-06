use crate::env::resource::database::Query;
use ::derive_more::Debug as DebugMore;
use ::serde::{Deserialize, Serialize};
use ::sqlx::PgPool;

mod fetch_password_hash;
mod fetch_user_by_id;
mod fetch_user_for_login;
mod insert_user;

pub use fetch_password_hash::{FetchPasswordHash, FetchPasswordHashError};
// pub use fetch_user_by_id::FetchUserById;
pub use fetch_user_for_login::FetchUserForLogin;
pub use insert_user::{InsertUser, InsertUserError};
