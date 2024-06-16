use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Json};
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::setting_model::SettingModel;
use crate::models::token_claim_model::LoggedInUser;

pub async fn setting_all_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>
) -> Result<Json<Vec<SettingModel>>> {
    println!("->> {:<12} - setting_all_api_handler", "HANDLER");


    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_setting"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    Ok(Json(state.setting_service.all(&state.db).await?))
}
