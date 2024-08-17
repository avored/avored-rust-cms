use std::sync::Arc;
use serde::Serialize;
use crate::{ error::Result };
use axum::{Extension, extract::Path, Json};
use axum::extract::State;
use crate::avored_state::AvoRedState;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

pub async fn delete_page_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    Path(page_id): Path<String>,
    state: State<Arc<AvoRedState>>
) -> Result<Json<RemovedPageResponse>> {
    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("page_delete"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }
    let page_result = state
        .page_service
        .find_by_id(&state.db, page_id.clone())
        .await;
    match page_result {
        Ok(_) => {
            state.page_service.remove_by_id(&state.db, &page_id).await?;
            let response = RemovedPageResponse {
                status: true
            };
            Ok(Json(response))
        },
        Err(error) => {
            match error {
                Error::Generic(ref message) if message == "no record found" => {
                    return Err(Error::NotFound("Page not found".to_string()));
                },
                _ => {
                    let mut errors: Vec<ErrorMessage> = vec![];
                    let error_message = ErrorMessage {
                        key: String::from("email"),
                        message: String::from("bad request error")
                    };
                    errors.push(error_message);
                    let error_response = ErrorResponse {
                        status: false,
                        errors
                    };
                    return Err(Error::BadRequestError(error_response))
                }
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RemovedPageResponse {
    pub status: bool,
}