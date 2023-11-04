use std::sync::Arc;

use crate::models::admin_user_model::AdminUserModel;
use crate::models::page_model::PageModel;
use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;
use crate::models::component_model::ComponentModel;

pub async fn edit_page_handler(
    session: AvoRedSession,
    Path(page_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - edit_page_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let mut page_model = state
        .page_service
        .find_by_id(&state.db, page_id)
        .await?;

    let components = state.component_service.all(&state.db).await?;
    println!("Components: {components:?}");

    page_model.content = page_model.content.replace("\r\n", "\r");

    let view_model = EditPageViewModel {
        logged_in_user,
        page_model,
        components
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("page/edit-page", &view_model)
        .expect("there is an issue with handlebar rendering page/edit-page.hbs template");

    Ok(Html(html))
}

#[derive(Serialize)]
pub struct EditPageViewModel {
    pub logged_in_user: AdminUserModel,
    pub page_model: PageModel,
    pub components: Vec<ComponentModel>
}
