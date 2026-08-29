use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    avored_state::AppState,
    core::application::dtos::LoginCommand,
};

#[axum::debug_handler(state = AppState)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginCommand>,
) -> axum::response::Response {
    let result = state.auth_use_case.auth(payload).await;

    if !result.authenticated {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "message": "Invalid credentials"
            })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "token": result.token,
            "user": {
                "id": result.user.id,
                "name": result.user.name,
                "email": result.user.email,
            },
            "authenticated": true,
        })),
    )
        .into_response()
}
