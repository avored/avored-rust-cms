use crate::avored_state::AvoRedState;
use crate::error::Error;
use crate::models::token_claim_model::{LoggedInUser, TokenClaims};
use crate::services::security_audit_service::SecurityEventType;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::{middleware::Next, Json};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use std::env;
use std::sync::Arc;
use tonic::Status;

#[derive(Debug, Serialize, Default)]
pub struct ErrorResponse {
    pub status: bool,
    pub message: String,
}

/// Enhanced JWT authentication middleware with comprehensive audit logging
pub async fn audit_enhanced_jwt_authentication(
    State(state): State<Arc<AvoRedState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let start_time = std::time::Instant::now();

    // Extract request metadata for audit logging
    let ip_address = extract_client_ip(&req);
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    let endpoint = req.uri().path().to_string();
    let method = req.method().to_string();

    match req.headers().get("authorization") {
        Some(auth_header) => {
            let auth_value = match auth_header.to_str() {
                Ok(value) => value,
                Err(_) => {
                    // Log failed authentication attempt - invalid header
                    let _ = log_authentication_event(
                        state,
                        None,
                        None,
                        ip_address.clone(),
                        user_agent.clone(),
                        Some(endpoint),
                        Some(method),
                        SecurityEventType::AuthenticationFailure,
                        Some("Invalid authorization header format".to_string()),
                    )
                    .await;

                    let json_error = ErrorResponse {
                        status: false,
                        message: "Invalid authorization header format".to_string(),
                    };
                    return Err((StatusCode::BAD_REQUEST, Json(json_error)));
                }
            };

            let jwt_token = match env::var("AVORED_JWT_SECRET") {
                Ok(token) => token,
                Err(_) => {
                    // Log configuration error
                    let _ = log_authentication_event(
                        state,
                        None,
                        None,
                        ip_address.clone(),
                        user_agent.clone(),
                        Some(endpoint),
                        Some(method),
                        SecurityEventType::SecurityViolation,
                        Some("JWT secret configuration missing".to_string()),
                    )
                    .await;

                    let json_error = ErrorResponse {
                        status: false,
                        message: "Authentication configuration error".to_string(),
                    };
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_error)));
                }
            };

            let token = match auth_value.strip_prefix("Bearer ") {
                Some(token) => token,
                None => {
                    // Log failed authentication attempt - invalid token format
                    let _ = log_authentication_event(
                        state,
                        None,
                        None,
                        ip_address.clone(),
                        user_agent.clone(),
                        Some(endpoint),
                        Some(method),
                        SecurityEventType::AuthenticationFailure,
                        Some("Invalid token format - Bearer prefix missing".to_string()),
                    )
                    .await;

                    let json_error = ErrorResponse {
                        status: false,
                        message: "Invalid token format".to_string(),
                    };
                    return Err((StatusCode::BAD_REQUEST, Json(json_error)));
                }
            };

            let claims = match decode::<TokenClaims>(
                token,
                &DecodingKey::from_secret(jwt_token.as_ref()),
                &Validation::default(),
            ) {
                Ok(token_data) => token_data.claims,
                Err(jwt_error) => {
                    // Log failed authentication attempt - invalid token
                    let error_message = match jwt_error.kind() {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired",
                        jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                        jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                            "Invalid token signature"
                        }
                        _ => "Token validation failed",
                    };

                    let _ = log_authentication_event(
                        state,
                        None,
                        None,
                        ip_address.clone(),
                        user_agent.clone(),
                        Some(endpoint),
                        Some(method),
                        SecurityEventType::AuthenticationFailure,
                        Some(format!("JWT validation failed: {}", error_message)),
                    )
                    .await;

                    let json_error = ErrorResponse {
                        status: false,
                        message: "Invalid or expired token".to_string(),
                    };
                    return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
                }
            };

            // Create logged in user
            let logged_in_user = LoggedInUser {
                id: claims.sub.clone(),
                name: claims.name.clone(),
                email: claims.email.clone(),
                demo_data_status: true, // You might want to check this properly
                admin_user_model: claims.admin_user_model.clone(),
            };

            // Generate session ID for this request
            let session_id = generate_session_id();

            // Log successful authentication
            let _ = log_authentication_event(
                state.clone(),
                Some(claims.sub.clone()),
                Some(session_id.clone()),
                ip_address.clone(),
                user_agent.clone(),
                Some(endpoint),
                Some(method),
                SecurityEventType::AuthenticationSuccess,
                Some(format!("User {} successfully authenticated", claims.email)),
            )
            .await;

            // Add user and session info to request extensions
            req.extensions_mut().insert(logged_in_user);
            req.extensions_mut().insert(session_id);

            // Process the request
            let response = next.run(req).await;

            // Log request completion time for performance monitoring
            let duration = start_time.elapsed();
            if duration.as_millis() > 1000 {
                // Log slow requests as suspicious activity
                let _ = log_authentication_event(
                    state,
                    Some(claims.sub),
                    None,
                    ip_address,
                    user_agent,
                    None,
                    None,
                    SecurityEventType::SuspiciousActivity,
                    Some(format!("Slow request detected: {}ms", duration.as_millis())),
                )
                .await;
            }

            Ok(response)
        }
        None => {
            // Log failed authentication attempt - no token provided
            let _ = log_authentication_event(
                state,
                None,
                None,
                ip_address,
                user_agent,
                Some(endpoint),
                Some(method),
                SecurityEventType::AuthenticationFailure,
                Some("No authorization header provided".to_string()),
            )
            .await;

            let json_error = ErrorResponse {
                status: false,
                message: "You are not logged in, please provide token".to_string(),
            };
            Err((StatusCode::UNAUTHORIZED, Json(json_error)))
        }
    }
}

/// Enhanced gRPC authentication middleware with audit logging
pub async fn audit_enhanced_grpc_auth(
    state: Arc<AvoRedState>,
    mut req: tonic::Request<()>,
) -> Result<tonic::Request<()>, Status> {
    let ip_address = extract_grpc_client_ip(&req);
    let ip_address_clone = ip_address.clone();
    let user_agent = req
        .metadata()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    let user_agent_clone = user_agent.clone();

    match req.metadata().get("authorization") {
        Some(auth_header) => {
            let auth_value = auth_header.to_str().map_err(|_| {
                // Log invalid header format
                let state_clone = state.clone();
                tokio::spawn(async move {
                    let _ = log_authentication_event(
                        state_clone,
                    None,
                    None,
                    ip_address_clone,
                    user_agent_clone,
                    None,
                    None,
                        SecurityEventType::AuthenticationFailure,
                        Some("Invalid gRPC authorization header format".to_string()),
                    ).await;
                });
                Status::unauthenticated("Invalid authorization header format")
            })?;

            let jwt_token = env::var("AVORED_JWT_SECRET").map_err(|_| {
                let state_clone = state.clone();
                let ip_address_clone2 = ip_address.clone();
                let user_agent_clone2 = user_agent.clone();
                tokio::spawn(async move {
                    let _ = log_authentication_event(
                        state_clone,
                        None,
                        None,
                        ip_address_clone2,
                        user_agent_clone2,
                    None,
                    None,
                        SecurityEventType::SecurityViolation,
                        Some("JWT secret configuration missing in gRPC".to_string()),
                    ).await;
                });
                Status::internal("Authentication configuration error")
            })?;

            let token = auth_value.strip_prefix("Bearer ").ok_or_else(|| {
                let state_clone = state.clone();
                let ip_address_clone3 = ip_address.clone();
                let user_agent_clone3 = user_agent.clone();
                tokio::spawn(async move {
                    let _ = log_authentication_event(
                        state_clone,
                        None,
                        None,
                        ip_address_clone3,
                        user_agent_clone3,
                    None,
                    None,
                        SecurityEventType::AuthenticationFailure,
                        Some("Invalid gRPC token format - Bearer prefix missing".to_string()),
                    ).await;
                });
                Status::unauthenticated("Invalid token format")
            })?;

            let claims = decode::<TokenClaims>(
                token,
                &DecodingKey::from_secret(jwt_token.as_ref()),
                &Validation::default(),
            )
            .map_err(|jwt_error| {
                let error_message = match jwt_error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired",
                    jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                    jsonwebtoken::errors::ErrorKind::InvalidSignature => "Invalid token signature",
                    _ => "Token validation failed",
                };

                tokio::spawn(log_authentication_event(
                    state.clone(),
                    None,
                    None,
                    ip_address.clone(),
                    user_agent.clone(),
                    None,
                    None,
                    SecurityEventType::AuthenticationFailure,
                    Some(format!("gRPC JWT validation failed: {}", error_message)),
                ));

                Status::unauthenticated("Invalid or expired token")
            })?
            .claims;

            // Log successful gRPC authentication
            tokio::spawn(log_authentication_event(
                state,
                Some(claims.sub.clone()),
                None,
                ip_address,
                user_agent,
                None,
                None,
                SecurityEventType::AuthenticationSuccess,
                Some(format!(
                    "gRPC user {} successfully authenticated",
                    claims.email
                )),
            ));

            req.extensions_mut().insert(claims);
            Ok(req)
        }
        None => {
            tokio::spawn(log_authentication_event(
                state,
                None,
                None,
                ip_address,
                user_agent,
                None,
                None,
                SecurityEventType::AuthenticationFailure,
                Some("No gRPC authorization header provided".to_string()),
            ));

            Err(Status::unauthenticated("No valid auth token"))
        }
    }
}

// Helper functions

async fn log_authentication_event(
    state: Arc<AvoRedState>,
    user_id: Option<String>,
    session_id: Option<String>,
    ip_address: String,
    user_agent: Option<String>,
    endpoint: Option<String>,
    method: Option<String>,
    event_type: SecurityEventType,
    message: Option<String>,
) -> Result<(), Error> {
    let metadata = message.map(|msg| {
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "event_message".to_string(),
            surrealdb::sql::Value::from(msg),
        );
        map
    });

    let _ = state
        .security_audit_service
        .log_security_event(
            &state.db.0,
            &state.db.1,
            user_id,
            session_id,
            ip_address,
            user_agent,
            endpoint,
            method,
            event_type,
            metadata,
        )
        .await;

    Ok(())
}

fn extract_client_ip(req: &Request<Body>) -> String {
    // Try to get real IP from various headers
    req.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
        })
        .or_else(|| {
            req.headers()
                .get("cf-connecting-ip")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

fn extract_grpc_client_ip(req: &tonic::Request<()>) -> String {
    req.metadata()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            req.metadata()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

fn generate_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    format!("session_{}", rng.random::<u64>())
}
