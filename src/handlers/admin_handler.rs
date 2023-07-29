use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn admin_handler() -> impl IntoResponse {
    Json(json!({"AvoRed": "admin handler"})).into_response()
}
