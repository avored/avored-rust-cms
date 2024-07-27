use std::sync::Arc;

use crate::error::Error;
use crate::models::page_model::{CreatableComponentContentModel, CreatableComponentElementContentModel, CreatablePageModel, PageModel, CreatablePageComponentElementDataModel};
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

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("page_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    if !error_messages.is_empty() {
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
            name: payload_component_content.name,
            identifier: payload_component_content.identifier,
            elements: vec![],
        };

        for  payload_component_elements_data in  payload_component_content.elements {
            let mut payload_element_data_model_options: Vec<CreatablePageComponentElementDataModel> = Vec::new();
            let  payload_element_options_data = payload_component_elements_data.element_data.unwrap_or(Vec::new());

            for payload_component_element_data_option in payload_element_options_data {
                let creatable_page_element_option_data = CreatablePageComponentElementDataModel {
                    label: payload_component_element_data_option.label,
                    value: payload_component_element_data_option.value,
                };
                payload_element_data_model_options.push(creatable_page_element_option_data);
            }

            let creatable_component_element_content = CreatableComponentElementContentModel {
                name: payload_component_elements_data.name,
                identifier: payload_component_elements_data.identifier,
                element_type: payload_component_elements_data.element_type,
                element_content: payload_component_elements_data.element_content,
                element_data: payload_element_data_model_options
            };

            creatable_component_content_model.elements.push(creatable_component_element_content);
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
