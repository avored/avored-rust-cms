use std::sync::Arc;

use crate::models::page_model::{CreatablePageModel, PageModel};
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{extract::State, Json};
use crate::api::rest_api::handlers::page::request::store_page_request::StorePageRequest;


pub async fn store_page_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StorePageRequest>,
) -> Result<Json<PageModel>> {
    let validation_error_list = payload.validate_errors()?;

    println!("Validation error list: {:?}", validation_error_list);

    let creatable_page = CreatablePageModel {
        name: payload.name,
        identifier: payload.identifier,
        content: payload.content,
        logged_in_username: "admin@admin.com".to_string(),
    };

    let created_page_model = state
        .page_service
        .create_page(&state.db, creatable_page)
        .await?;


    Ok(Json(created_page_model))
}
