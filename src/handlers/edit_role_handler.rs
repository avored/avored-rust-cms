use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;
use std::sync::Arc;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::models::role_model::RoleModel;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn edit_role_handler(
    state: State<Arc<AvoRedState>>,
    Path(role_id): Path<String>,
    mut session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let role_model = match state
        .role_service
        .find_by_id(&state.datastore, &state.database_session, role_id)
        .await
    {
        Ok(model) => model,
        Err(_) => RoleModel::empty(),
    };

    let mut view_model = EditRoleViewModel::new();
    view_model.role_model = role_model;

    view_model.validation_error_name = session.get("validation_error_name").unwrap_or_default();
    view_model.validation_error_identifier = session
        .get("validation_error_identifier")
        .unwrap_or_default();

    session.remove("validation_error_name");
    session.remove("validation_error_identifier");

    // println!("AdminUSER {:?}", view_model.admin_user_model);

    // let validation_error_full_name = session.get("validation_error_full_name");

    // view_model.validation_error_full_name = match validation_error_full_name {
    //     Some(message) => message,
    //     None => String::from(""),
    // };

    view_model.logged_in_user = logged_in_user;

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("role/edit-role", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()

    // Json(admin_users).into_response()
}

#[derive(Serialize)]
pub struct EditRoleViewModel {
    logged_in_user: AdminUser,
    role_model: RoleModel,
    validation_error_name: String,
    validation_error_identifier: String,
}

impl EditRoleViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        let empty_role_model = RoleModel::empty();
        EditRoleViewModel {
            logged_in_user,
            role_model: empty_role_model,
            validation_error_name: String::from(""),
            validation_error_identifier: String::from(""),
        }
    }
}
