use crate::avored_state::AppState;
use crate::core::domain::entities::user::TokenClaims;
use crate::core::domain::entities::ApiErrorMessageResponse;
use crate::error::unauthorized;
use crate::infrastructure::middleware::api_key_middleware::hash_api_key;
use axum::body::Body;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{http::Request, middleware::Next, Json};
use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing::error;

fn bearer_token(headers: &HeaderMap) -> Result<&str, (StatusCode, Json<ApiErrorMessageResponse>)> {
    let Some(header_value) = headers.get("authorization") else {
        return Err(unauthorized("You are not logged in, please provide token"));
    };
    let Ok(auth_value) = header_value.to_str() else {
        return Err(unauthorized(
            "Authentication failed: Invalid authorization header",
        ));
    };
    auth_value
        .strip_prefix("Bearer ")
        .ok_or_else(|| unauthorized("Authentication failed: Invalid authorization header"))
}

/// Middleware to require JWT authentication for incoming requests
pub async fn check_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorMessageResponse>)> {
    let token = bearer_token(req.headers())?;
    let jwt_token = &state.config.jwt_secret_key;
    let claims = match decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_token.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data.claims,
        Err(jwt_error) => {
            let error_message = match jwt_error.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired",
                jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                jsonwebtoken::errors::ErrorKind::InvalidSignature => "Invalid token signature",
                _ => "Token validation failed",
            };

            return Err(unauthorized(format!(
                "Authentication failed: {error_message}"
            )));
        }
    };

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Middleware to require API-key authentication for incoming customer requests
#[allow(clippy::result_large_err)]
pub async fn check_customer_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let api_key = bearer_token(req.headers()).map_err(IntoResponse::into_response)?;

    match state
        .api_key_use_case
        .find_by_hash_public(&hash_api_key(api_key))
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err(unauthorized("Authentication failed: Invalid API key").into_response())
        }
        Err(db_error) => {
            error!(error = ?db_error, "Customer API key lookup failed");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorMessageResponse {
                    status: false,
                    message: "An internal server error occurred.".to_string(),
                }),
            )
                .into_response());
        }
    }

    // @todo improve this to a proper user claims for api users.
    // May be we can have backend to select a user (created, updated and deleted by user email)
    // and map it with domain name as well.
    let now = chrono::Utc::now().timestamp() as usize;
    req.extensions_mut().insert(TokenClaims {
        sub: "users:api-user".to_string(),
        name: "API User".to_string(),
        email: "api-user@avored.local".to_string(),
        iat: now,
        exp: now + 3600,
    });

    Ok(next.run(req).await)
}
