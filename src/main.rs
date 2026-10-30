#![recursion_limit = "256"]

use avored_rust_cms::{api::rest_api_routes::rest_api_routes, avored_state::AppState, error};

#[tokio::main]
async fn main() -> error::Result<()> {
    let _log_guard = avored_rust_cms::infrastructure::logging::initialize_tracing()
        .map_err(|error| error::Error::Generic(format!("failed to initialize tracing: {error}")))?;
    match avored_rust_cms::infrastructure::logging::cleanup_expired_logs(std::path::Path::new(
        "public/log",
    )) {
        Ok(removed) if removed > 0 => {
            tracing::info!(removed, retention_days = 14, "removed expired log files");
        }
        Ok(_) => {}
        Err(error) => {
            tracing::error!(%error, "failed to clean up expired log files");
        }
    }
    avored_rust_cms::infrastructure::logging::spawn_log_cleanup();

    let conf = leptos::config::get_configuration(None)?;
    tracing::info!("application configuration loaded");
    let leptos_options = conf.leptos_options;

    let state = AppState::new(leptos_options).await?;
    tracing::info!("application state initialized");

    let app: axum::Router = rest_api_routes(state)?;

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(address = %listener.local_addr()?, "server listening");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
