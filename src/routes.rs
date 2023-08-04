use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    avored_state::AvoRedState,
    handlers::{admin_handler::admin_handler, home_handler::home_handler},
};

pub fn routes(state: AvoRedState) -> Router {
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/admin", get(admin_handler))
        .with_state(Arc::new(state));

    app
}
