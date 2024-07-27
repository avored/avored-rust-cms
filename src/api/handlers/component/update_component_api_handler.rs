use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
};
use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::api::handlers::component::request::update_component_request::{UpdatableComponentElementDataRequest, UpdateComponentRequest};
use crate::error::Error;
use crate::models::component_model::{ComponentElementDataModel, ComponentModel, UpdatableComponentElementModel, UpdatableComponentModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;

pub async fn update_component_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(component_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateComponentRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_component_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("component_edit"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    let mut updatable_elements: Vec<UpdatableComponentElementModel> = vec![];
    for payload_element in payload.elements {

        let mut updatable_element_data: Vec<ComponentElementDataModel> = vec![];

        let updatable_element_data_requests: Vec<UpdatableComponentElementDataRequest> = payload_element.element_data.unwrap_or_else(std::vec::Vec::new);

        for update_element_data_request in updatable_element_data_requests {
            let create_element_data_option = ComponentElementDataModel {
                label: update_element_data_request.label,
                value: update_element_data_request.value
            };
            updatable_element_data.push(create_element_data_option);
        }

        let updatable_component_element_model = UpdatableComponentElementModel {
            name: payload_element.name,
            identifier: payload_element.identifier,
            element_type: payload_element.element_type,
            element_data: Some(updatable_element_data)
        };
        updatable_elements.push(updatable_component_element_model);
    }

    let updatable_component_model = UpdatableComponentModel {
        id: component_id,
        name: payload.name,
        logged_in_username: logged_in_user.email.clone(),
        elements: updatable_elements
    };
    let updated_component_model = state
        .component_service
        .update_component(&state.db, updatable_component_model)
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