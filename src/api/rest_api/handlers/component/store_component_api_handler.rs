use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{extract::State, Json};
use serde::Serialize;
use crate::api::rest_api::handlers::component::request::store_component_request::StoreComponentRequest;
use crate::models::component_model::{ComponentModel, CreatableComponent};
use crate::models::field_model::CreatableFieldModel;


pub async fn store_component_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreComponentRequest>,
) -> Result<Json<CreatedComponentResponse>> {
    // let _validation_error_list = payload.validate_errors()?;

    // println!("Payload: {:?}", payload.clone());
    let creatable_component = CreatableComponent {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: "admin@admin.com".to_string(),
    };

    let mut created_component = state
        .component_service
        .create_component(&state.db, creatable_component)
        .await?;

    // println!("Created component {:?}", created_component.clone());

    for payload_field in payload.fields {
        let creatable_field = CreatableFieldModel {
            name: payload_field.name,
            identifier: payload_field.identifier,
            field_type: payload_field.field_type,
            logged_in_username: "admin@admin.com".to_string(),
        };

        // println!("creatable_field: {creatable_field:?}");

        let created_field = state
            .field_service
            .create_field(&state.db, creatable_field)
            .await?;

        // println!("Created component {:?}", created_component.clone());
        // println!("Created Field {:?}", created_field.clone());

        state
            .component_service
            .attach_component_with_field(
                &state.db,
                created_component.clone(),
                created_field.clone(),
                "admin@admin.com".to_string(),
            )
            .await?;
        // println!("ATTACHED: {:?}", created_field.clone());

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