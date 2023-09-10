use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession, models::{role_model::RoleModel, admin_user_model::AdminUserModel},
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn edit_role_handler(
    session: AvoRedSession,
    Path(role_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - edit_role_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let role_model = state.role_service.find_by_id(&state.db, role_id).await?;

    let view_model = EditRoleViewModel {
        logged_in_user,
        role_model,
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/edit-role", &view_model)
        .expect("there is an issue with handlerbar rendering admin-user/edit-role.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct EditRoleViewModel {
    pub logged_in_user: AdminUserModel,
    pub role_model: RoleModel,
}
