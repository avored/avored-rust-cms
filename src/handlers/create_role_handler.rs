use std::sync::Arc;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn create_role_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };
   
    let mut view_model = CreateRoleViewModel::new();

    view_model.logged_in_user = logged_in_user;
    view_model.validation_error_name = session.get("validation_error_name").unwrap_or(String::from(""));
    view_model.validation_error_identifier = session.get("validation_error_identifier").unwrap_or(String::from(""));

    session.remove("validation_error_name");
    session.remove("validation_error_identifier");


    let handlebars = &state.handlebars;

    let html = handlebars
        .render("role/create-role", &view_model)
        .expect("there is an issue while loading the create role template");

    Html(html).into_response()
}

#[derive(Serialize)]
pub struct CreateRoleViewModel {
    logged_in_user: AdminUser,
    validation_error_name: String,
    validation_error_identifier: String,
}

impl CreateRoleViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        CreateRoleViewModel {
            logged_in_user,
            validation_error_name: String::from(""),
            validation_error_identifier: String::from(""),
        }
    }
}
