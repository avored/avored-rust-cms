use std::sync::Arc;

use crate::{avored_state::AvoRedState, error::Result};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn setup_avored_handler(state: State<Arc<AvoRedState>>) -> Result<impl IntoResponse> {
    println!("->> {:<12} - setup_avored_handler", "HANDLER");
    let view_model = SetupViewModel {};

    // let admin_users = state.admin_user_service.all_admin_users(&state.db).await?;
    let handlebars = &state.handlebars;

    let html = handlebars
        .render("setup/index", &view_model)
        .expect("there is an issue with handlerbar rendering setup/index.hbs template");

    Ok(Html(html))
}

#[derive(Serialize)]
pub struct SetupViewModel {}
