//!
//! # Open Hack
//!
//! Keeping developers in contact with each other.
//!

// ====================[ Public Exports ]====================

pub use openhack::OpenHack;

/// Library Error / Result
pub use anyhow::{Error, Result};

// ====================[ Public Modules ]====================

/// All the possible state around calls into the library
pub mod context;

/// Configuration on the running library
pub mod config;

/// All interactions to the library are via commands
pub mod command;

/// Shared structures and traits
pub mod common;

/// Public facing data structures
pub mod entity;

// ====================[ Private Modules ]===================

/// Runtime environment
mod env;

/// Library Application Interface
mod openhack;

#[cfg(test)]
mod support;
