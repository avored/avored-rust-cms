use std::sync::Arc;
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Request, StatusCode};
use axum::Json;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::ErrorResponse;
use crate::models::setting_model::SettingModel;

pub async fn validate_cms_authentication (
    state: State<Arc<AvoRedState>>,
    cookie_jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {

    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        match auth_value.strip_prefix("Bearer ") {
                            Some(auth) => Some(auth.to_owned()),
                            _ => None
                        }
                    } else {
                        None
                    }
                })
        });


    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: false,
            message: "please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;
    let cms_token_setting_model = state
        .setting_service
        .find_by_identifier(&state.db, String::from("auth_cms_token"))
        .await.unwrap_or_else(|_err|  {
            SettingModel::default()
        });

    if cms_token_setting_model.value.is_empty() | cms_token_setting_model.value.ne(&token) {
        let json_error = ErrorResponse {
            status: false,
            message: "please provide valid token".to_string(),
        };
        return Err((StatusCode::UNAUTHORIZED, Json(json_error)))
    };

    Ok(next.run(req).await)
}