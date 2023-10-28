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
use axum::extract::Query;
use serde::Serialize;
use crate::api::asset::requests::asset_table_query::AssetTableQuery;
use crate::models::asset_model::AssetPagination;

pub async fn asset_table_handler(
    session: AvoRedSession,
    Query(query_param): Query<AssetTableQuery>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - asset_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let current_page = query_param.page.unwrap_or(1);
    let assets_pagination = state
        .asset_service
        .paginate(&state.db, current_page)
        .await?;

    let view_model = AssetTableViewModel {
        logged_in_user,
        assets_pagination
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
    pub assets_pagination: AssetPagination
}
