use std::sync::Arc;
use axum::extract::{State, Path};
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::models::role_model::RoleModel;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn show_role_handler(
    state: State<Arc<AvoRedState>>,
    Path(role_id): Path<String>,
    session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let role_model = match state.role_service.find_by_id(&state.datastore, &state.database_session, role_id).await {
        Ok(model) => model,
        Err(_) => RoleModel::empty(),
    };

    let mut view_model = ShowRoleViewModel::new();
    view_model.role_model = role_model;

    view_model.logged_in_user = logged_in_user;
    let handlebars = &state.handlebars;
    let html = handlebars
        .render("role/show-role", &view_model)
        .expect("there is an issue while loading the show role template");

    Html(html).into_response()
}

#[derive(Serialize)]
pub struct ShowRoleViewModel {
    logged_in_user: AdminUser,
    role_model: RoleModel,
}

impl ShowRoleViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        let empty_role_model = RoleModel::empty();
        ShowRoleViewModel {
            logged_in_user,
            role_model: empty_role_model
        }
    }
}
