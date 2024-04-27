use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::models::component_model::ComponentModel;

pub async fn fetch_component_api_handler(
    AxumPath(component_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_component_api_handler", "HANDLER");

    let component_model = state
        .component_service
        .find_by_id(&state.db, component_id)
        .await?;
    let response = FetchPageResponse {
        status: true,
        component_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct FetchPageResponse {
    pub status: bool,
    pub component_model: ComponentModel
}