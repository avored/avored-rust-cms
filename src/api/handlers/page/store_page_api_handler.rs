use std::sync::Arc;

use crate::error::Error;
use crate::models::page_model::{CreatableComponentContentModel, CreatableComponentFieldContentModel, CreatablePageModel, PageModel};
use crate::models::validation_error::ErrorResponse;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use serde::Serialize;
use crate::api::handlers::page::request::store_page_request::StorePageRequest;
use crate::models::token_claim_model::LoggedInUser;


pub async fn store_page_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StorePageRequest>,
) -> Result<Json<CreatedPageResponse>> {
    let error_messages = payload.validate()?;

    if error_messages.len() > 0 {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }
    
    let mut  creatable_page = CreatablePageModel {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email.clone(),
        component_contents: vec![]
    };

    //

    for payload_component_content in payload.components_content {
        let mut  creatable_component_content_model = CreatableComponentContentModel {
            id: payload_component_content.id,
            name: payload_component_content.name,
            identifier: payload_component_content.identifier,
            fields: vec![],
        };

        for  payload_component_fields_data in  payload_component_content.fields {
            let creatable_component_field_content = CreatableComponentFieldContentModel {
                id: payload_component_fields_data.id,
                name: payload_component_fields_data.name,
                identifier: payload_component_fields_data.identifier,
                field_type: payload_component_fields_data.field_type,
                field_content: payload_component_fields_data.field_content,
            };

            creatable_component_content_model.fields.push(creatable_component_field_content);
        }

        creatable_page.component_contents.push(creatable_component_content_model);
    }

    let created_page_model = state
        .page_service
        .create_page(&state.db, creatable_page, logged_in_user)
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
