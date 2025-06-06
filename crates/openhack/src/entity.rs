//!
//! # Open Hack Entities
//!
//! This module contains all of the defined public facing
//! data structures used.  The difference between this and
//! [crate::common] resides in the fact that they are locally
//! defined by this crate.
//!

// ====================[ Public Exports ]====================

pub use email::Email;
pub use user::User;
pub use user_id::UserId;

// ====================[ Crate Exports ]=====================

pub(crate) use password::Password;
pub(crate) use password_hash::PasswordHash;
pub(crate) use user_hash::UserHash;

// ====================[ Private Modules ]===================

mod email;
mod password;
mod password_hash;
mod user;
mod user_hash;
mod user_id;
