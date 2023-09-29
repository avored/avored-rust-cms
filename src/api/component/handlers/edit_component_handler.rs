use std::sync::Arc;

use crate::models::admin_user_model::AdminUserModel;
use crate::models::component_model::ComponentModel;
use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn edit_component_handler(
    session: AvoRedSession,
    Path(component_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - edit_component_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let component_model = state
        .component_service
        .find_by_id(&state.db, component_id)
        .await?;

    let mut view_model = EditComponentViewModel::default();
    view_model.logged_in_user = logged_in_user;
    view_model.component_model = component_model;

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("component/edit-component", &view_model)
        .expect("there is an issue with handlebar rendering component/edit-component.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct EditComponentViewModel {
    pub logged_in_user: AdminUserModel,
    pub component_model: ComponentModel,
}
