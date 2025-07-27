use crate::error::Error;
use crate::models::token_claim_model::{LoggedInUser, TokenClaims};
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{http::Request, middleware::Next, Json};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use std::env;
use tonic::Status;

#[derive(Debug, Serialize, Default)]
/// error response struct
pub struct ErrorResponse {
    /// status of the response
    pub status: bool,
    /// message of the response
    pub message: String,
}

/// Middleware to require JWT authentication for incoming requests
pub async fn require_jwt_authentication(
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match req.headers().get("authorization") {
        Some(t) => {
            let auth_value = t
                .to_str()
                .map_err(|_e| Status::unavailable("authorization header value is not valid string"))
                .unwrap();

            let jwt_token = &env::var("AVORED_JWT_SECRET")
                .map_err(|_| Error::ConfigMissing("AVORED_JWT_SECRET".to_string()))
                .unwrap();
            let token = auth_value
                .strip_prefix("Bearer ")
                .map(|auth| auth.to_owned());
            let claims = match decode::<TokenClaims>(
                &token.unwrap_or_default(),
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

                    let json_error = ErrorResponse {
                        status: false,
                        message: format!("Authentication failed: {}", error_message),
                    };
                    return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
                }
            };

            let file_exist = true;
            let logged_in_user = LoggedInUser {
                id: claims.sub,
                name: claims.name,
                email: claims.email,
                demo_data_status: file_exist,
                admin_user_model: claims.admin_user_model,
            };
            req.extensions_mut().insert(logged_in_user);

            Ok(next.run(req).await)
        }
        _ => {
            let json_error = ErrorResponse {
                status: false,
                message: "You are not logged in, please provide token".to_string(),
            };
            Err((StatusCode::BAD_REQUEST, Json(json_error)))
        }
    }
    // let token = cookie_jar
    //     .get("token")
    //     .map(|cookie| cookie.value().to_string())
    //     .or_else(|| {
    //         req.headers()
    //             .get(header::AUTHORIZATION)
    //             .and_then(|auth_header| auth_header.to_str().ok())
    //             .and_then(|auth_value| {
    //                 if auth_value.starts_with("Bearer ") {
    //                     match auth_value.strip_prefix("Bearer ") {
    //                         Some(auth) => Some(auth.to_owned()),
    //                         _ => None,
    //                     }
    //                 } else {
    //                     None
    //                 }
    //             })
    //     });
    //
    // // let mut is_token_valid = false;
    //
    // let token = token.ok_or_else(|| {
    //     let json_error = ErrorResponse {
    //         status: false,
    //         message: "You are not logged in, please provide token".to_string(),
    //     };
    //     (StatusCode::UNAUTHORIZED, Json(json_error))
    // })?;
    //
    // let secret = state.config.jwt_secret_key.clone();
    //
    // let claims = decode::<TokenClaims>(
    //     &token,
    //     &DecodingKey::from_secret(secret.as_ref()),
    //     &Validation::default(),
    // )
    // .map_err(|_| {
    //     let json_error = ErrorResponse {
    //         status: false,
    //         message: "Invalid token".to_string(),
    //     };
    //     (StatusCode::UNAUTHORIZED, Json(json_error))
    // })?
    // .claims;
    // let file_exist = tokio::fs::try_exists("public/install_demo")
    //     .await
    //     .unwrap_or(false);
    //
    // let logged_in_user = LoggedInUser {
    //     id: claims.sub,
    //     name: claims.name,
    //     email: claims.email,
    //     demo_data_status: file_exist,
    //     admin_user_model: claims.admin_user_model,
    // };
    //
    // req.extensions_mut().insert(logged_in_user);
    //
    // Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::middleware::Next;
    use axum::response::Response;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::env;
    use chrono::{Duration, Utc};

    // Helper function to create a test token
    fn create_test_token(expired: bool) -> String {
        let now = Utc::now();
        let exp = if expired {
            (now - Duration::minutes(10)).timestamp() as usize // Expired 10 minutes ago
        } else {
            (now + Duration::minutes(60)).timestamp() as usize // Valid for 60 minutes
        };

        let claims = TokenClaims {
            sub: "test-user-id".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            admin_user_model: crate::models::admin_user_model::AdminUserModel::default(),
            iat: now.timestamp() as usize,
            exp,
        };

        let secret = env::var("AVORED_JWT_SECRET").unwrap_or_else(|_| "test-secret".to_string());
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }

    // Mock next function for testing
    async fn mock_next(_req: Request<Body>) -> Result<Response<Body>, std::convert::Infallible> {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("success"))
            .unwrap())
    }

    #[tokio::test]
    async fn test_expired_token_should_return_unauthorized() {
        // Set up environment
        env::set_var("AVORED_JWT_SECRET", "test-secret");

        // Create request with expired token
        let expired_token = create_test_token(true);
        let req = Request::builder()
            .header("authorization", format!("Bearer {}", expired_token))
            .body(Body::empty())
            .unwrap();

        // Create mock next function
        let next = mock_next;

        // Call the middleware
        let next = mock_next;
        let result = require_jwt_authentication(req, next).await;

        // After the fix, this should return Err with StatusCode::UNAUTHORIZED
        match result {
            Ok(_) => panic!("Expected error for expired token, but got success"),
            Err((status, json_response)) => {
                assert_eq!(status, StatusCode::UNAUTHORIZED, "Expected 401 Unauthorized for expired token");
                // Verify the error message contains information about token expiry
                let error_msg = format!("{:?}", json_response);
                assert!(error_msg.contains("Token expired") || error_msg.contains("Authentication failed"),
                       "Error message should indicate token expiry: {}", error_msg);
            }
        }
    }

    #[tokio::test]
    async fn test_valid_token_should_succeed() {
        // Set up environment
        env::set_var("AVORED_JWT_SECRET", "test-secret");

        // Create request with valid token
        let valid_token = create_test_token(false);
        let req = Request::builder()
            .header("authorization", format!("Bearer {}", valid_token))
            .body(Body::empty())
            .unwrap();

        // Create mock next function
        let next = mock_next;

        // Call the middleware
        let result = require_jwt_authentication(req, next).await;

        // Should succeed with valid token
        assert!(result.is_ok(), "Expected success with valid token");
    }

    #[tokio::test]
    async fn test_missing_token_should_return_bad_request() {
        // Create request without authorization header
        let req = Request::builder()
            .body(Body::empty())
            .unwrap();

        // Create mock next function
        let next = Next::new(mock_next);

        // Call the middleware
        let result = require_jwt_authentication(req, next).await;

        // Should return error for missing token
        match result {
            Ok(_) => panic!("Expected error for missing token, but got success"),
            Err((status, _)) => {
                assert_eq!(status, StatusCode::BAD_REQUEST, "Expected 400 Bad Request for missing token");
            }
        }
    }
}
