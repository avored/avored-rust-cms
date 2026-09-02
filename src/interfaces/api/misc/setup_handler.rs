use axum::{extract::State, Json};
use rust_i18n::t;
use crate::core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse};
use crate::core::domain::entities::user::StorableUser;
use crate::core::domain::extensions::string_extension::StringExtension;
use crate::error::Result;
use crate::avored_state::AppState;


pub async fn setup_handler(
    State(state): State<AppState>,
    Json(payload): Json<SetupCommand>,
) -> Result<Json<SetupResponse>> {
    println!("->> {:<12} - setup_handler", "REST_API_HANDLER");

    let locale = "en";

    payload.validate(locale)?;
    let storable_user = SetupCommand::from(&payload);
    
    let result = state.misc_use_case.setup(storable_user).await?;

    let res = SetupResponse {
        success: result,
    };

    Ok(Json(res))
}



#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SetupCommand {
    pub name: String,
    pub email: String,
    pub password: String
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SetupResponse {
    pub success: bool
}

impl SetupCommand {
    fn from(&self) -> StorableUser {
        StorableUser {
            name: self.name.clone(),
            email: self.email.clone(),
            password: self.password.clone()
        }    
    }
}

impl SetupCommand {
    fn validate (&self, locale: &str) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.name.is_required()? {
            let error_message = ErrorMessageResponse {
                key: String::from("name"),
                message: t!("required", locale = locale, attribute = t!("name", locale = locale)).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }
        if !self.email.is_required()? {
            let error_message = ErrorMessageResponse {
                key: String::from("email"),
                message: t!("required", locale = locale, attribute = t!("email", locale = locale)).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }
        if !self.password.is_required()? {
            let error_message = ErrorMessageResponse {
                key: String::from("password"),
                message: t!("required", locale = locale, attribute = t!("password", locale = locale)).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        if !valid {
            let error_response = ErrorResponse {
                status: valid,
                errors,
            };

            return Err(crate::error::Error::BadRequest(error_response));
        }
              

        Ok(errors)

    }
}

