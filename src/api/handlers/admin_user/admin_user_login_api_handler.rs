use std::sync::Arc;
use axum::extract::State;
use axum::http::{header, Response};
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;
use crate::api::handlers::admin_user::request::authenticate_admin_user_request::AuthenticateAdminUserRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::admin_user_model::AdminUserModel;
use crate::models::token_claim_model::TokenClaims;
use crate::models::validation_error::ErrorResponse;


/// Login Admin User
///
/// Authenticate admin user
#[utoipa::path(
    post,
    path = "/api/login",
    responses(
        (status = 200, description = "JSON file", body = LoginResponseData)
    ),
    request_body = AuthenticateAdminUserRequest,
)]
pub async fn admin_user_login_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<AuthenticateAdminUserRequest>,
) -> Result<Json<LoginResponseData>> {
    println!("->> {:<12} - admin_user_login_api_handler", "HANDLER");

    let error_messages = payload.validate()?;

    if error_messages.len() > 0 {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    let admin_user_model = state
        .admin_user_service
        .find_by_email(&state.db, payload.email.to_owned())
        .await?;

    let is_password_match: bool = state
        .admin_user_service
        .compare_password(
            payload.password.clone(),
            admin_user_model.password.clone()
        )?;

    if !is_password_match {
        return Err(Error::AuthenticationError);
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: admin_user_model.clone().id,
        name: admin_user_model.clone().full_name,
        email:admin_user_model.clone().email,
        admin_user_model: admin_user_model.clone(),
        exp,
        iat,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&state.config.jwt_secret_key.as_ref()),
    ).unwrap();
    let cookie = Cookie::build("token")
        .path("/")
        // .max_age(Duration::h)
        .same_site(SameSite::Lax)
        .http_only(true);
    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    let response_data = LoginResponseData {
        status: true,
        data: token,
        admin_user: admin_user_model
    };


    Ok(Json(response_data))
}


#[derive(Serialize, ToSchema)]
pub struct LoginResponseData {
    status: bool,
    data: String,
    admin_user: AdminUserModel
}