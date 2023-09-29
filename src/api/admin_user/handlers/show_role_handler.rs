use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
    models::{admin_user_model::AdminUserModel, role_model::RoleModel},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn show_role_handler(
    session: AvoRedSession,
    Path(role_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - show_role_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let role_model = state.role_service.find_by_id(&state.db, role_id).await?;

    let view_model = ShowRoleViewModel {
        logged_in_user,
        role_model,
    };

    // let admin_user_model = state.ad`

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/show-role", &view_model)
        .expect("there is an issue with handlerbar rendering admin-user/show-role.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct ShowRoleViewModel {
    pub logged_in_user: AdminUserModel,
    pub role_model: RoleModel,
}
