use crate::error::Result;
use crate::models::token_claim_model::LoggedInUser;
use crate::responses::ApiResponse;
use axum::{response::IntoResponse, Extension, Json};

pub async fn logged_in_user_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - logged_in_user_api_handler", "HANDLER");

    let response = ApiResponse {
        status: true,
        data: logged_in_user,
    };

    Ok(Json(response))
}
