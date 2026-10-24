use axum::{extract::State, Json};

use crate::avored_state::AppState;
use crate::core::application::dtos::setting_dto::SettingsGroupResponse;
use crate::error::Result;

pub async fn fetch_settings_handler(
    State(state): State<AppState>,
) -> Result<Json<SettingsGroupResponse>> {
    let settings = state.setting_use_case.get_all_settings_grouped().await?;
    Ok(Json(settings))
}
