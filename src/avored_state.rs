use leptos::config::LeptosOptions;
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}
