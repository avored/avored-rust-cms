use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{extract::State, Json, response::IntoResponse};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

pub async fn sent_contact_us_email_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<SentContactUsEmailRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - sent_contact_us_email_handler", "HANDLER");
    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequest(error_response));
    }

    let template = &state.template;
    let contact_email_status = state
        .cms_service
        .sent_contact_us_email(template, payload)
        .await?;

    let res = SentContactUsEmailResponse {
        status: contact_email_status
    };
    // let res = page_model.convert_to_response()?;

    Ok(Json(res))
}


#[derive(Serialize, Debug)]
#[cfg_attr(test, derive(Deserialize, Eq, PartialEq, Copy, Clone, Default))]
pub struct SentContactUsEmailResponse {
    pub status: bool,
}



#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SentContactUsEmailRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub message: String,
    pub phone: String,
}

impl SentContactUsEmailRequest {
    fn validate(&self) -> Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.email.is_empty() {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Email is a required field")
            };

            errors.push(error_message);
        }

        if ! EmailAddress::is_valid(&self.email) {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Invalid email address")
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
