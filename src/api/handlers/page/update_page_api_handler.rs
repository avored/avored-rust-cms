use std::sync::Arc;

use crate::error::Error;
use crate::models::page_model::{CreatablePageField, NewCreatablePageModel, NewPageModel, NewUpdatablePageModel, UpdatablePageField};
use crate::{
    api::handlers::page::request::update_page_request::UpdatePageRequest,
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;

pub async fn update_page_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(page_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdatePageRequest>,
) -> Result<Json<ApiResponse<NewPageModel>>> {
    println!("->> {:<12} - update_page_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("page_edit"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    let page_model = state
        .page_service
        .find_by_id(&state.db, page_id)
        .await?;

    let mut updatable_page = NewUpdatablePageModel {
        id: page_model.id,
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.name.clone(),
        page_fields: vec![],
        created_at: page_model.created_at,
        created_by: page_model.created_by
    };

    for  payload_page_field in  payload.page_fields {
        let page_field_model = UpdatablePageField {
            name: payload_page_field.name,
            identifier: payload_page_field.identifier,
            data_type: payload_page_field.data_type,
            field_type: payload_page_field.field_type,
            field_content: payload_page_field.field_content
        };
        updatable_page.page_fields.push(page_field_model);
    }

    let created_page_model = state
        .page_service
        .update_page(&state.db, updatable_page)
        .await?;

    let response = ApiResponse {
        status: true,
        data: created_page_model
    };

    Ok(Json(response))
}


// #[derive(Serialize, Debug)]
// pub struct UpdatablePageResponse {
//     pub status: bool,
//     pub page_model: NewPageModel
// }
