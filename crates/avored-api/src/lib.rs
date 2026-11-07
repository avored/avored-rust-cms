#![recursion_limit = "144"]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod api;
pub mod avored_state;
pub mod infrastructure;
pub mod interfaces;
pub mod providers;

pub use avored_core::{core, error};

rust_i18n::i18n!("../../resources/locales");
