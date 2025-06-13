//!
//! # Open Hack
//!
//! Keeping developers in contact with each other.
//!

// ====================[ Public Exports ]====================

pub use config::Config;
pub use context::Context;
pub use openhack::OpenHack;

// ====================[ Public Modules ]====================

/// All non-idempotent requests to the library
pub mod command;

/// Shared structures and traits
pub mod common;

/// Public facing data structures
pub mod entity;

/// All idempotent requests to the library
pub mod report;

// ====================[ Private Modules ]===================

/// Configuration on the running library
mod config;

/// All the possible state around calls into the library
mod context;

/// Runtime environment
mod env;

/// Library Application Interface
mod openhack;

/// Testing Support
#[cfg(test)]
mod support;
