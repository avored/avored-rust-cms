use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn edit_admin_user_handler(
    mut session: AvoRedSession,
    Path(admin_user_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - edit_admin_user_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let admin_user_model = state
        .admin_user_service
        .find_by_id(&state.db, admin_user_id)
        .await?;
    let success_message = session
        .get("success_message")
        .unwrap_or(String::from(""));
        session.remove("success_message");

    let view_model = EditAdminUserHandlerViewModel {
        logged_in_user,
        admin_user_model,
        success_message
    };

    // let admin_user_model = state.ad`

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/edit-admin-user", &view_model)
        .expect("there is an issue with handlerbar rendering admin-user/edit-admin-user.hbs template");
    
    session
        .insert("success_message", "Admin User added successfully!")
        .expect("Could not store the validation errors into session.");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct EditAdminUserHandlerViewModel {
    pub logged_in_user: AdminUserModel,
    pub admin_user_model: AdminUserModel,
    pub success_message: String
}
