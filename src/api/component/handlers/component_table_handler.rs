use std::sync::Arc;

use crate::{
    api::component::requests::component_table_query::ComponentTableQuery,
    avored_state::AvoRedState,
    error::Result,
    models::{admin_user_model::AdminUserModel, component_model::ComponentPagination},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn component_table_handler(
    mut session: AvoRedSession,
    Query(query_param): Query<ComponentTableQuery>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - component_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };
    let current_page = query_param.page.unwrap_or(1);
    let component_pagination = state
        .component_service
        .paginate(&state.db, current_page)
        .await?;
    let success_message = session.get("success_message").unwrap_or(String::from(""));
    session.remove("success_message");

    let view_model = ComponentViewModel {
        logged_in_user,
        component_pagination,
        success_message,
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("component/component-table", &view_model)
        .expect(
            "there is an issue with handlebar rendering component/component-table.hbs template",
        );

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct ComponentViewModel {
    pub logged_in_user: AdminUserModel,
    pub component_pagination: ComponentPagination,
    pub success_message: String,
}
