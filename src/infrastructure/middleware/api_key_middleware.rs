use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;
use sha2::{Digest, Sha256};

use crate::avored_state::AppState;
use crate::core::domain::entities::ApiErrorMessageResponse;

fn unauthorized(message: &str) -> (StatusCode, Json<ApiErrorMessageResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiErrorMessageResponse {
            status: false,
            message: message.to_string(),
        }),
    )
}

/// Middleware to validate API key from the `X-Api-Key` header on v1 endpoints
pub async fn check_api_key_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorMessageResponse>)> {
    if let Some(header_val) = req.headers().get("x-api-key") {
        let plain_secret = header_val
            .to_str()
            .map_err(|_| unauthorized("Invalid X-Api-Key header"))?;

        let mut hasher = Sha256::new();
        hasher.update(plain_secret.as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());

        let api_key = state
            .api_key_use_case
            .find_by_hash_public(&key_hash)
            .await
            .map_err(|_| unauthorized("Invalid API key"))?
            .ok_or_else(|| unauthorized("Invalid API key"))?;

        req.extensions_mut().insert(api_key.permissions);
        Ok(next.run(req).await)
    } else {
        Err(unauthorized("X-Api-Key header missing"))
    }
}
