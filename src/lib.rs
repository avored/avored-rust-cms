//! `AvoRed` Rust CMS Library
//!
//! This library provides the core functionality for the `AvoRed` CMS,
//! including security services, authentication, and data models.


/// api module
pub mod api;

/// `avored_state` module
pub mod avored_state;

/// error module
pub mod error;

/// extensions module
pub mod extensions;

/// middleware module
pub mod middleware;

/// models module
pub mod models;

/// providers module
pub mod providers;

/// repositories module
pub mod repositories;

/// requests module
pub mod requests;
/// Security services and authentication utilities.
pub mod security;
/// services module
pub mod services;

// Re-export commonly used items
pub use error::{Error, Result};

/// Constants per page
pub const PER_PAGE: i64 = 10;

// Re-export the i18n macro
rust_i18n::i18n!("locales");
