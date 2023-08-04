use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handlers::{admin_handler::admin_handler, home_handler::home_handler}, state::State};

pub fn routes(state: State) -> Router {
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/admin", get(admin_handler))
        .with_state(Arc::new(state));

    app
}
