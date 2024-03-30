use std::sync::Arc;

use crate::models::page_model::{CreatableComponentContentModel, CreatableComponentFieldContentModel, CreatablePageModel, PageModel};
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{extract::State, Json};
use serde::Serialize;
use crate::api::rest_api::handlers::page::request::store_page_request::StorePageRequest;


pub async fn store_page_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StorePageRequest>,
) -> Result<Json<CreatedPageResponse>> {
    // let _validation_error_list = payload.validate_errors()?;

    // println!("Payload SENT: {:?}", payload);

    let mut  creatable_page = CreatablePageModel {
        name: payload.name,
        identifier: payload.identifier,
        content: "test content".to_string(), // ideally we should not need this one
        logged_in_username: "admin@admin.com".to_string(),
        component_contents: vec![]
    };

    //

    for payload_component_content in payload.components_content {
        let mut  creatable_component_content_model = CreatableComponentContentModel {
            id: payload_component_content.id,
            name: payload_component_content.name,
            identifier: payload_component_content.identifier,
            component_fields_content: vec![],
        };

        for  payload_component_fields_data in  payload_component_content.component_fields_content {
            let creatable_component_field_content = CreatableComponentFieldContentModel {
                id: payload_component_fields_data.id,
                name: payload_component_fields_data.name,
                identifier: payload_component_fields_data.identifier,
                field_type: payload_component_fields_data.field_type,
                field_content: payload_component_fields_data.field_content,
            };

            creatable_component_content_model.component_fields_content.push(creatable_component_field_content);
        }

        creatable_page.component_contents.push(creatable_component_content_model);
    }

    // println!("Payload GENERATED: {:?}", creatable_page);

    let created_page_model = state
        .page_service
        .create_page(&state.db, creatable_page)
        .await?;
    let response = CreatedPageResponse {
        status: true,
        page_model: created_page_model
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct CreatedPageResponse {
    pub status: bool,
    pub page_model: PageModel
}