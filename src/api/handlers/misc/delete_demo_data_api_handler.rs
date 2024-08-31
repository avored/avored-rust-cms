use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
};
use axum::{extract::State, Json, response::IntoResponse, Extension};
use serde::Serialize;
use crate::models::token_claim_model::LoggedInUser;

pub async fn delete_demo_data_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - install_demo_data_api_handler", "HANDLER");

    let sql = "
        DELETE components WHERE identifier='hero-component';
        DELETE components WHERE identifier='repository-component';
        DELETE components WHERE identifier='key-features-component';
        DELETE components WHERE identifier='contact-us-component';
        DELETE pages WHERE identifier='home-page';
    ";

    let vars = BTreeMap::from([
        ("email".into(), logged_in_user.email.into()),
    ]);

    let (ds, ses) = &state.db;

    let responses = ds.execute(sql, ses, Some(vars)).await?;

    println!("{responses:?}");
    println!();

    tokio::fs::remove_file("public/install_demo").await?;

    let response = DemoDataViewModel {
        status: true
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct DemoDataViewModel {
    pub status: bool,
}
