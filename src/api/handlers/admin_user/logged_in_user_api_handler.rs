use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::State, Json, response::IntoResponse};
use serde::Serialize;
use crate::models::admin_user_model::AdminUserModel;
use crate::models::token_claim_model::LoggedInUser;

pub async fn logged_in_user_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - logged_in_user_api_handler {:?}", "HANDLER", logged_in_user);

    let admin_user_model = state
        .admin_user_service
        .find_by_id(&state.db, logged_in_user.id)
        .await?;
    let response = FetchAdminUserResponse {
        status: true,
        admin_user_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct FetchAdminUserResponse {
    pub status: bool,
    pub admin_user_model: AdminUserModel
}