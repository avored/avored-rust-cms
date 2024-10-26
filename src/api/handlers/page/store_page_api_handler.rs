use std::sync::Arc;

use crate::error::Error;
use crate::models::page_model::{CreatablePageField, NewCreatablePageModel, NewPageModel};
use crate::models::validation_error::ErrorResponse;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use crate::api::handlers::page::request::store_page_request::StorePageRequest;
use crate::models::token_claim_model::LoggedInUser;
use crate::responses::ApiResponse;

pub async fn store_page_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StorePageRequest>,
) -> Result<Json<ApiResponse<NewPageModel>>> {
    println!("->> {:<12} - store_page_api_handler", "HANDLER");
    let error_messages = payload.validate(&state).await?;

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("page_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequest(error_response));
    }

    let mut creatable_page = NewCreatablePageModel {
        name: payload.name,
        identifier: payload.identifier,
        status: payload.status,
        logged_in_username: logged_in_user.name.clone(),
        page_fields: vec![]
    };

    for  payload_page_field in  payload.page_fields {
        let creatable_page_field_model = CreatablePageField {
            name: payload_page_field.name,
            identifier: payload_page_field.identifier,
            data_type: payload_page_field.data_type,
            field_type: payload_page_field.field_type,
            field_content: payload_page_field.field_content,
            field_data: payload_page_field.field_data
        };
        creatable_page.page_fields.push(creatable_page_field_model);
    }

    let created_page_model = state
        .page_service
        .new_create_page(&state.db, creatable_page)
        .await?;
    // println!("PAge payload: {:?}", payload);
    // let created_page_model = NewPageModel::default();

    let response = ApiResponse {
        status: true,
        data: created_page_model
    };

    Ok(Json(response))
}
