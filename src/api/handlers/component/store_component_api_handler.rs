use std::sync::Arc;

use crate::api::handlers::component::request::store_component_request::{
    CreatableComponentElementDataRequest, StoreComponentRequest,
};
use crate::error::Error;
use crate::models::component_model::{
    ComponentElementDataModel, ComponentModel, CreatableComponent, CreatableComponentElementModel,
};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::{avored_state::AvoRedState, error::Result};
use axum::{extract::State, Extension, Json};
use serde::Serialize;

pub async fn store_component_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreComponentRequest>,
) -> Result<Json<CreatedComponentResponse>> {
    println!("->> {:<12} - store_component_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("component_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    let mut creatable_elements: Vec<CreatableComponentElementModel> = vec![];
    for payload_element in payload.elements {
        let mut creatable_element_data: Vec<ComponentElementDataModel> = vec![];

        let create_element_data_requests: Vec<CreatableComponentElementDataRequest> =
            payload_element
                .element_data
                .unwrap_or_else(std::vec::Vec::new);

        for create_element_data_request in create_element_data_requests {
            let create_element_data_option = ComponentElementDataModel {
                label: create_element_data_request.label,
                value: create_element_data_request.value,
            };
            creatable_element_data.push(create_element_data_option);
        }

        let creatable_component_element_model = CreatableComponentElementModel {
            name: payload_element.name,
            identifier: payload_element.identifier,
            element_type: payload_element.element_type,
            element_data_type: payload_element.element_data_type,
            element_data: Some(creatable_element_data),
        };
        creatable_elements.push(creatable_component_element_model);
    }

    let creatable_component = CreatableComponent {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email.clone(),
        elements: creatable_elements,
    };

    let created_component = state
        .component_service
        .create_component(&state.db, creatable_component)
        .await?;

    let created_response = CreatedComponentResponse {
        status: true,
        component_model: created_component,
    };

    Ok(Json(created_response))
}

#[derive(Serialize, Debug)]
pub struct CreatedComponentResponse {
    pub status: bool,
    pub component_model: ComponentModel,
}
