use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use crate::api::handlers::component::request::component_table_query::ComponentTableQuery;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::component_model::ComponentPagination;
use crate::models::token_claim_model::LoggedInUser;

pub async fn component_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Query(query_param): Query<ComponentTableQuery>,
) -> Result<Json<ComponentPagination>> {
    println!("->> {:<12} - component_table_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("component_table"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let current_page = query_param.page.unwrap_or(1);
    let component_pagination = state.component_service.paginate(&state.db, current_page).await?;

    Ok(Json(component_pagination))
}
