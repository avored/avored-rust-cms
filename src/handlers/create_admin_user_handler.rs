use std::sync::Arc;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn create_admin_user_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };
   
    let mut view_model = CreateAdminUserHandlerViewModel::new();

    let validation_error_full_name = session.get("validation_error_full_name");
    let validation_error_email = session.get("validation_error_email");
    let validation_error_password = session.get("validation_error_password");
    let validation_error_confirmation_password = session.get("validation_error_confirmation_password");

    view_model.validation_error_full_name = match validation_error_full_name {
        Some(message) => message,
        None => String::from(""),
    };
    view_model.validation_error_email = match validation_error_email {
        Some(message) => message,
        None => String::from(""),
    };
    view_model.validation_error_password = match validation_error_password {
        Some(message) => message,
        None => String::from(""),
    };
    view_model.validation_error_confirmation_password =
        match validation_error_confirmation_password {
            Some(message) => message,
            None => String::from(""),
        };
    
    view_model.logged_in_user = logged_in_user;

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("admin-user/create-admin-user", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()

    // Json(admin_users).into_response()
}

#[derive(Serialize)]
pub struct CreateAdminUserHandlerViewModel {
    logged_in_user: AdminUser,
    validation_error_full_name: String,
    validation_error_email: String,
    validation_error_password: String,
    validation_error_confirmation_password: String
}

impl CreateAdminUserHandlerViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        CreateAdminUserHandlerViewModel {
            logged_in_user,
            validation_error_full_name: String::from(""),
            validation_error_email: String::from(""),
            validation_error_password: String::from(""),
            validation_error_confirmation_password: String::from("")
        }
    }
}
