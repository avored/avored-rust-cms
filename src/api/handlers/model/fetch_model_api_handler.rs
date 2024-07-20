use std::sync::Arc;

use crate::models::model_model::ModelModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;

pub async fn fetch_model_api_handler(
    AxumPath(model_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_model_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_model"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let model_model = state
        .model_service
        .find_by_id(&state.db, model_id)
        .await?;
    let response = FetchModelResponse {
        status: true,
        model_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct FetchModelResponse {
    pub status: bool,
    pub model_model: ModelModel
}