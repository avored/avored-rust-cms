use axum::{routing::get, Router};

use crate::handlers::{admin_handler::admin_handler, home_handler::home_handler};

pub fn app_route() -> Router {
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/admin", get(admin_handler));

    app
}
