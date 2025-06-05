use crate::env::resource::database::Query;
use ::derive_more::Debug as DebugMore;
use ::serde::{Deserialize, Serialize};
use ::sqlx::PgPool;

mod insert_user;

pub use insert_user::{InsertUser, InsertUserError};
