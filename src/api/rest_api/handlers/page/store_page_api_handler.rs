use std::sync::Arc;

use crate::models::page_model::{ComponentContentModel, CreatablePageModel, PageModel};
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
    let _validation_error_list = payload.validate_errors()?;

    // println!("Validation error list: {:?}", validation_error_list);

    let mut  creatable_page = CreatablePageModel {
        name: payload.name,
        identifier: payload.identifier,
        content: "test content".to_string(), // ideally we should not need this one
        logged_in_username: "admin@admin.com".to_string(),
        component_content: vec![]
    };

    for payload_component_content in payload.component_content {
        let creatable_component_content_model = ComponentContentModel {
            id: payload_component_content.id,
            name: payload_component_content.name,
            identifier: payload_component_content.identifier,
            content: payload_component_content.content,
        };

        creatable_page.component_content.push(creatable_component_content_model);
    }


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