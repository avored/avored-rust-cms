use std::sync::Arc;

use crate::{avored_state::AvoRedState, error::Result};

use crate::error::Error;
use crate::models::component_model::ComponentModel;
use crate::models::token_claim_model::LoggedInUser;
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Serialize;

pub async fn fetch_component_api_handler(
    AxumPath(component_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_component_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_component"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let component_model = state
        .component_service
        .find_by_id(&state.db, component_id)
        .await?;
    let response = FetchPageResponse {
        status: true,
        component_model,
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct FetchPageResponse {
    pub status: bool,
    pub component_model: ComponentModel,
}
