use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use serde::Serialize;
use crate::api::handlers::component::request::store_component_request::{FieldDataRequest, StoreComponentRequest};
use crate::error::Error;
use crate::models::component_model::{ComponentModel, CreatableComponent};
use crate::models::field_model::{CreatableFieldDataModel, CreatableFieldModel};
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
    let creatable_component = CreatableComponent {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email.clone(),
    };

    let mut created_component = state
        .component_service
        .create_component(&state.db, creatable_component)
        .await?;
    
    for payload_field in payload.fields {
        let mut creatable_field_data: Vec<CreatableFieldDataModel> = vec![];

        let create_field_data_requests: Vec<FieldDataRequest> = payload_field.field_data.unwrap_or_else(std::vec::Vec::new);

        for create_field_data_request in create_field_data_requests {
            let create_field_data_option = CreatableFieldDataModel {
                label: create_field_data_request.label,
                value: create_field_data_request.value
            };
            creatable_field_data.push(create_field_data_option);
        }

        let creatable_field = CreatableFieldModel {
            name: payload_field.name,
            identifier: payload_field.identifier,
            field_type: payload_field.field_type,
            field_data: Some(creatable_field_data),
            logged_in_username: logged_in_user.email.clone(),
        };

        let created_field = state
            .field_service
            .create_field(&state.db, creatable_field)
            .await?;

        state
            .component_service
            .attach_component_with_field(
                &state.db,
                created_component.clone(),
                created_field.clone(),
                logged_in_user.email.clone(),
            )
            .await?;
        created_component.fields.push(created_field);
    }
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