use crate::api::handlers::admin_user::request::authenticate_admin_user_request::AuthenticateAdminUserRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::admin_user_model::AdminUserModel;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use rust_i18n::t;
use utoipa::ToSchema;

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
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    match state.admin_user_service
        .auth_admin_user(&state.db, payload, &state.config.jwt_secret_key)
        .await {
        // Success Response
        Ok(login_response) => Ok(Json(login_response)),

        // matching error and return response
        Err(e) => match e {
            Error::ModelNotFound(_e)  |
            Error::Authentication(_e) => {
                let mut errors: Vec<ErrorMessage> = vec![];
                let error_message = ErrorMessage {
                    key: String::from("email"),
                    message: t!("email_address_password_not_match").to_string(),
                };
                errors.push(error_message);
                let error_response = ErrorResponse {
                    status: false,
                    errors
                };

                Err(Error::BadRequest(error_response))
            },

            _ => Err(Error::Generic("500 Internal Server Error".to_string()))
        }
    }
}

#[derive(Serialize, ToSchema, Deserialize, Debug)]
pub struct LoginResponseData {
    pub status: bool,
    pub data: String,
    pub admin_user: AdminUserModel,
}

#[cfg(test)]
mod tests {
    use crate::api::rest_api_routes::tests::{get_axum_app, send_post_request};
    use crate::error::Result;
    use axum::body::Body;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_admin_user_login_api_handler() -> Result<()> {
        let (app, _state) = get_axum_app().await.unwrap();
        //@todo do a post request to a setup
        // then do a post request with username and password

        let payload = Body::from(
            r#"{
                    "email": "admin@admin.com",
                    "password": "admin123"
                }"#,
        );

        let response = app
            .oneshot(send_post_request("/api/setup", payload))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();

        println!("response: {:?}", body);

        Ok(())
    }
}
