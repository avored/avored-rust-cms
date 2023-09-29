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

pub async fn show_admin_user_handler(
    session: AvoRedSession,
    Path(admin_user_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - show_admin_user_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let admin_user_model = state
        .admin_user_service
        .find_by_id(&state.db, admin_user_id)
        .await?;

    let view_model = ShowAdminUserHandlerViewModel {
        logged_in_user,
        admin_user_model,
    };

    // let admin_user_model = state.ad`

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/show-admin-user", &view_model)
        .expect(
            "there is an issue with handlerbar rendering admin-user/show-admin-user.hbs template",
        );

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct ShowAdminUserHandlerViewModel {
    pub logged_in_user: AdminUserModel,
    pub admin_user_model: AdminUserModel,
}
