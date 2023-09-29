use std::sync::Arc;

use crate::models::component_model::ComponentModel;
use crate::{
    avored_state::AvoRedState, error::Result, models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn show_component_handler(
    session: AvoRedSession,
    Path(component_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - show_role_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let component_model = state
        .component_service
        .find_by_id(&state.db, component_id)
        .await?;

    // println!("COM: {component_model:?}");
    let view_model = ShowComponentViewModel {
        logged_in_user,
        component_model,
    };

    // let admin_user_model = state.ad`

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("component/show-component", &view_model)
        .expect("there is an issue with handlebar rendering component/show-component.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct ShowComponentViewModel {
    pub logged_in_user: AdminUserModel,
    pub component_model: ComponentModel,
}
