use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use crate::api::rest_api::handlers::component::request::component_table_query::ComponentTableQuery;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::component_model::ComponentPagination;

pub async fn component_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Query(query_param): Query<ComponentTableQuery>,
) -> Result<Json<ComponentPagination>> {
    println!("->> {:<12} - component_table_api_handler", "HANDLER");

    let current_page = query_param.page.unwrap_or(1);
    let component_pagination = state.component_service.paginate(&state.db, current_page).await?;

    Ok(Json(component_pagination))
}
