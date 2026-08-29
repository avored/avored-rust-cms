use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    core::{
        application::{dtos::LoginCommand, use_cases::LoginUser},
    },
    infrastructure::auth::DemoAuthRepository,
};

pub async fn login_handler(Json(payload): Json<LoginCommand>) -> impl IntoResponse {
    let repository = DemoAuthRepository;
    let use_case = LoginUser::new(repository);
    let result = use_case.execute(payload);

    if !result.authenticated {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "message": "Invalid credentials"
            })),
        );
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
}
