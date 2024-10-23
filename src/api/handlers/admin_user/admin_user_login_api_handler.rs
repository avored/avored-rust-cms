use std::sync::Arc;
use axum::extract::State;
use axum::http::{header, Response};
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
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

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequest(error_response));
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
        return Err(Error::Authentication);
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
        &EncodingKey::from_secret(state.config.jwt_secret_key.as_ref()),
    )?;
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


#[derive(Serialize, ToSchema, Deserialize, Debug)]
pub struct LoginResponseData {
    pub status: bool,
    pub data: String,
    pub admin_user: AdminUserModel
}


#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::StatusCode;
    use tower::ServiceExt;
    use crate::api::rest_api_routes::tests::{ get_axum_app, send_post_request};
    use crate::error::Result;


    #[tokio::test]
    async fn test_admin_user_login_api_handler() -> Result<()>
    {
        let (app, _state) = get_axum_app().await.unwrap();
        //@todo do a post request to a setup
        // then do a post request with username and password

        let payload = Body::from(
            r#"{
                    "email": "admin@admin.com",
                    "password": "admin123"
                }"#,
        );

        let response = app.oneshot(send_post_request("/api/setup", payload)).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();

        println!("response: {:?}", body);

        Ok(())
    }
}
