use axum::response::{IntoResponse, Redirect};

pub async fn home_handler() -> impl IntoResponse {
    Redirect::to("/admin").into_response()
}
