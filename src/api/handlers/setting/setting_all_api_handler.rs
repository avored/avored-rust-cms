use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::setting_model::SettingModel;

pub async fn setting_all_api_handler(
    state: State<Arc<AvoRedState>>
) -> Result<Json<Vec<SettingModel>>> {
    println!("->> {:<12} - setting_all_api_handler", "HANDLER");

    Ok(Json(state.setting_service.all(&state.db).await?))
}
