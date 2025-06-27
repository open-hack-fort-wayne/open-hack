//!
//! # OpenHack Queries
//!
//! Contains all of the [Query] objects available.  Any
//! access to the database should be done with these
//! adapter pattern data strcutures.
//!
#![allow(unused_imports)]

//-----------------------[ Private Exports ]------------------------

use crate::env::resource::database::Query;
use ::derive_more::Debug as DebugMore;
use ::serde::{Deserialize, Serialize};
use ::sqlx::PgPool;

//-----------------------[ Public Exports ]-------------------------

pub use fetch_event_by_id::FetchEventById;
pub use fetch_password_hash::{FetchPasswordHash, FetchPasswordHashError};
pub use fetch_user_by_id::FetchUserById;
pub use fetch_user_for_login::FetchUserForLogin;
pub use find_events::FindEvents;
pub use insert_event::InsertEvent;
pub use insert_user::{InsertUser, InsertUserError};
pub use update_event::UpdateEvent;
pub use update_event_for_owner::UpdateEventForCreator;
pub use upsert_event_rsvp::UpsertEventRsvp;

//-----------------------[ Private Moudles ]------------------------

mod fetch_event_by_id;
mod fetch_password_hash;
mod fetch_user_by_id;
mod fetch_user_for_login;
mod find_events;
mod insert_event;
mod insert_user;
mod update_event;
mod update_event_for_owner;
mod upsert_event_rsvp;
