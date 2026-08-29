use std::sync::Arc;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::providers::{
    avored_config_provider::AvoRedConfigProvider
};

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,

    /// database connection, and various services.
    // pub db: Arc<DB>,

    /// Configuration provider for `AvoRed`.
    pub config: Arc<AvoRedConfigProvider>,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
