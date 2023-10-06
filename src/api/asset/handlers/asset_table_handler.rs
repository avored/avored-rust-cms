use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
    models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn asset_table_handler(
    session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - asset_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let view_model = AssetTableViewModel {
        logged_in_user
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("asset/asset-table", &view_model)
        .expect("there is an issue with handlebar rendering asset/asset-table.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct AssetTableViewModel {
    pub logged_in_user: AdminUserModel,
}
