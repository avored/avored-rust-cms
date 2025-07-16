//! AvoRed Rust CMS Library
//!
//! This library provides the core functionality for the AvoRed CMS,
//! including security services, authentication, and data models.

pub mod api;
pub mod avored_state;
pub mod error;
pub mod extensions;
pub mod middleware;
pub mod models;
pub mod providers;
pub mod repositories;
pub mod requests;
pub mod security;
pub mod services;

// Re-export commonly used items
pub use error::{Error, Result};

// Constants
pub const PER_PAGE: i64 = 10;

// Re-export the i18n macro
rust_i18n::i18n!("locales");
