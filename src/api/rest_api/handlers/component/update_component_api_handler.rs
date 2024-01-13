use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
};
use axum::{extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::api::rest_api::handlers::component::request::update_component_request::UpdateComponentRequest;
use crate::models::component_model::{ComponentModel, UpdatableComponentModel};

pub async fn update_component_api_handler(
    AxumPath(component_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateComponentRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_component_api_handler", "HANDLER");


    let updateable_component_model = UpdatableComponentModel {
        id: component_id,
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: "admin@admin.com".to_string(),
    };
    let updated_component_model = state
        .component_service
        .update_component(&state.db, updateable_component_model)
        .await?;

    let response = UpdatedComponentResponse {
        status: true,
        component_model: updated_component_model
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct UpdatedComponentResponse {
    pub status: bool,
    pub component_model: ComponentModel
}