use std::sync::Arc;

use crate::{
    api::page::requests::page_table_query::PageTableQuery,
    avored_state::AvoRedState,
    error::Result,
    models::{admin_user_model::AdminUserModel, page_model::PagePagination},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn page_table_handler(
    mut session: AvoRedSession,
    Query(query_param): Query<PageTableQuery>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - page_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };
    let current_page = query_param.page.unwrap_or(1);
    let page_pagination = state.page_service.paginate(&state.db, current_page).await?;
    let success_message = session.get("success_message").unwrap_or(String::from(""));
    session.remove("success_message");

    let view_model = PageViewModel {
        logged_in_user,
        page_pagination,
        success_message,
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("page/page-table", &view_model)
        .expect("there is an issue with handlebar rendering page/page-table.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct PageViewModel {
    pub logged_in_user: AdminUserModel,
    pub page_pagination: PagePagination,
    pub success_message: String,
}
