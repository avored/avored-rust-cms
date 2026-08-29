use std::sync::Arc;

use avored_rust_cms::{
    api::rest_api_routes::rest_api_routes,
    avored_state::AppState,
    error,
    providers::{
        avored_config_provider::AvoRedConfigProvider,
        avored_database_provider::AvoRedDatabaseProvider,
    },
};

#[tokio::main]
async fn main() -> error::Result<()> {
    let conf = leptos::config::get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    let config = AvoRedConfigProvider::new()?;

    let _avored_database_provider = AvoRedDatabaseProvider::register(
        &config.database_folder,
        &config.database_namespace,
        &config.database_name,
    )
    .await?;

    // build our application with a route
    // let avored_state = Arc::new(AvoRedState::new(leptos_options.clone()).await?);
    // let database_provider = AvoRedDatabaseProvider::new(config.database_url.clone()).await?;

    let state = AppState {
        leptos_options,
        config: Arc::new(config),
    };

    let app: axum::Router = rest_api_routes(state)?;

    println!("listening on http://{}", 3000);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
