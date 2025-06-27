//! # Command Module
//!
//! All interactions with the [super::OpenHack] system
//! are encapsulated by data structures which implement
//! the [core::CommandExt] trait.
//!

//-----------------------[ Public Exports ]-------------------------

pub use core::CommandRunner;

//-----------------------[ Public Modules ]------------------------

pub mod create_event;
pub mod create_user;
pub mod login_user;
pub mod update_event;
pub mod upsert_rsvp;

//-----------------------[ Private Modules ]------------------------

mod core;
