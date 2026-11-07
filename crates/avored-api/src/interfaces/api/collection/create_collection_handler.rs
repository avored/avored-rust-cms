use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;

use crate::avored_state::AppState;
use crate::core::application::dtos::collection_dto::CreateCollectionCommand;
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct CreateCollectionQuery {
    pub entity_id: String,
}

pub async fn create_collection_handler(
    State(state): State<AppState>,
    Query(query): Query<CreateCollectionQuery>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Json(payload): Json<CreateCollectionCommand>,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    let response = state
        .entity_use_case
        .create_collection(&query.entity_id, payload.fields, &logged_in_user.email)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}
