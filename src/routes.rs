use std::sync::Arc;
use tower_http::services::ServeDir;
use axum::{routing::get, Router};

use crate::{
    avored_state::AvoRedState,
    handlers::{admin_handler::admin_handler, home_handler::home_handler},
};

pub fn routes(state: AvoRedState) -> Router {

    let public_static_service = ServeDir::new("public");

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/admin", get(admin_handler))
        .nest_service("/public", public_static_service)
        .with_state(Arc::new(state));

    app
}
