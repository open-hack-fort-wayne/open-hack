//!
//! # Open Hack Entities
//!
//! This module contains all of the defined public facing
//! data structures used.  The difference between this and
//! [crate::common] resides in the fact that they are locally
//! defined by this crate.
//!

// ====================[ Public Exports ]====================

pub use date_selection::DateSelection;
pub use email_address::EmailAddress;
pub use event::Event;
pub use event_id::EventId;
pub use event_rsvp::EventRsvp;
pub use next_page_results::NextPageResults;
pub use rsvp_status::RsvpStatus;
pub use user::User;
pub use user_id::UserId;

// ====================[ Crate Exports ]=====================

pub(crate) use password::Password;
pub(crate) use password_hash::PasswordHash;
pub(crate) use user_hash::UserHash;

// ====================[ Private Modules ]===================

mod date_selection;
mod email_address;
mod event;
mod event_id;
mod event_rsvp;
mod next_page_results;
mod password;
mod password_hash;
mod rsvp_status;
mod user;
mod user_hash;
mod user_id;
