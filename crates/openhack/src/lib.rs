//!
//! # Open Hack
//!
//! Keeping developers in contact with each other.
//!

/// All of possible state around calls into the library
pub mod context;

/// Configuration on the running library
pub mod config;

/// All interactions to the library are via commands
pub mod command;

/// Shared structures and traits
pub mod common;

/// Public facing data structures
pub mod entity;

/// Library Error / Result
pub use anyhow::{Error, Result};

/// The runtime environment
mod env;
