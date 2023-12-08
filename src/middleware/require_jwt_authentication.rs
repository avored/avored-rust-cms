use axum::{http::Request, Json, middleware::Next, response::{IntoResponse, Response}};
use axum::http::{header, StatusCode};
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn require_jwt_authentication<T>(
    cookie_jar: CookieJar,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, impl IntoResponse> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let mut is_token_valid = false;

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    });

    if is_token_valid {
        return Err(Redirect::to("/admin/login").into_response());
    }

    println!("TOKEN {token:?}");

    Ok(next.run(req).await)
}
