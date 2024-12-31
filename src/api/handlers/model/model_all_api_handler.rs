use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::model_model::ModelModel;
use crate::models::token_claim_model::LoggedInUser;
use axum::extract::State;
use axum::{Extension, Json};
use std::sync::Arc;
use crate::responses::ApiResponse;

pub async fn model_all_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>
) -> Result<Json<ApiResponse<Vec<ModelModel>>>> {
    println!("->> {:<12} - model_all_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("model_all"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }


    let models = state
        .model_service
        .all_models(&state.db)
        .await?;

    let response = ApiResponse {
        status: true,
        data: models,
    };

    Ok(Json(response))
}
