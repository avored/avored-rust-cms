use std::sync::Arc;

use crate::models::page_model::{PageModel, UpdatablePageModel};
use crate::{
    api::rest_api::handlers::page::request::update_page_request::UpdatePageRequest,
    avored_state::AvoRedState, error::Result
};

use axum::{extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;

pub async fn update_page_api_handler(
    AxumPath(page_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdatePageRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_page_api_handler", "HANDLER");

    let _validation_error_list = payload.validate_errors()?;

    // println!("Validation error list: {:?}", validation_error_list);

    let updateable_page_model = UpdatablePageModel {
        id: page_id,
        name: payload.name,
        identifier: payload.identifier,
        content: "updated test content".to_string(), // @todo remove this one
        logged_in_username: "admin@admin.com".to_string(),
    };
    let updated_page_model = state
        .page_service
        .update_page(&state.db, updateable_page_model)
        .await?;
    let response = UpdatablePageResponse {
        status: true,
        page_model: updated_page_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct UpdatablePageResponse {
    pub status: bool,
    pub page_model: PageModel
}