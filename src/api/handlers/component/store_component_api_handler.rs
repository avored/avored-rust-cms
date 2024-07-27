use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use serde::Serialize;
use crate::api::handlers::component::request::store_component_request::StoreComponentRequest;
use crate::error::Error;
use crate::models::component_model::{ComponentModel, CreatableComponent, CreatableComponentElementModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;


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

    let mut creatable_elements: Vec<CreatableComponentElementModel> = vec![];
    for payload_element in payload.elements {

        let creatable_component_element_model = CreatableComponentElementModel {
            name: payload_element.name
        };
        creatable_elements.push(creatable_component_element_model);
    }


    let creatable_component = CreatableComponent {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email.clone(),
        elements: creatable_elements
    };

    let created_component = state
        .component_service
        .create_component(&state.db, creatable_component)
        .await?;

    let created_response = CreatedComponentResponse {
        status: true,
        component_model: created_component
    };

    Ok(Json(created_response))
}

#[derive(Serialize, Debug)]
pub struct CreatedComponentResponse {
    pub status: bool,
    pub component_model: ComponentModel
}