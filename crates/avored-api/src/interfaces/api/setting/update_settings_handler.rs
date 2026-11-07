use axum::{extract::State, Json};

use crate::avored_state::AppState;
use crate::core::application::dtos::setting_dto::{SettingsGroupResponse, UpdateSettingsPayload};
use crate::error::Result;

pub async fn update_settings_handler(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettingsPayload>,
) -> Result<Json<SettingsGroupResponse>> {
    let updated = state.setting_use_case.update_settings(payload).await?;
    Ok(Json(updated))
}
