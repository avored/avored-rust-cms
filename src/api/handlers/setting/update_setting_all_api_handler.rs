use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Json};
use crate::api::handlers::setting::request::update_setting_request::UpdateSettingRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::setting_model::{SettingModel, UpdatableSettingModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;

pub async fn update_setting_all_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateSettingRequest>,
) -> Result<Json<Vec<SettingModel>>> {
    println!("->> {:<12} - update_setting_all_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("save_setting"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequest(error_response));
    }

    for updatable_setting in payload.settings {
        let updatable_setting_model = UpdatableSettingModel {
            id: updatable_setting.id,
            value: updatable_setting.value,
            logged_in_username: logged_in_user.email.clone()
        };
        state.setting_service.update_setting(&state.db, updatable_setting_model).await?;
    }


    Ok(Json(state.setting_service.all(&state.db).await?))
}
