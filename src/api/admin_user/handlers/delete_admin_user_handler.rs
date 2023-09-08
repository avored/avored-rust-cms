use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

pub async fn delete_admin_user_handler(
    _session: AvoRedSession,
    Path(_admin_user_id): Path<String>,
    _state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - delete_admin_user_handler", "HANDLER");

    // @todo add update service
    // let admin_user_model = state
    //     .admin_user_service
    //     .find_by_email(&state.db, String::from("admin@admin.com"))
    //     .await?;

    // let view_model = EditAdminUserHandlerViewModel {
    //     logged_in_user,
    //     admin_user_model,
    // };

    // let admin_user_model = state.ad`

    // let handlebars = &state.handlebars;
    // let html = handlebars
    //     .render("admin-user/edit-admin-user", &view_model)
    //     .expect("there is an issue with handlerbar rendering admin-user/table.hbs template");

    Ok(Redirect::to("/admin/admin-user").into_response())
}
