use rust_i18n::t;
use serde::{Deserialize, Serialize};

use crate::{
    core::domain::{
        entities::{
            error_message::{ApiErrorResponse, ErrorMessageResponse},
            UserModel,
        },
        extensions::string_extension::StringExtension,
    },
    error::{Error, Result},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

impl LoginCommand {
    pub fn validate(&self, locale: &str) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.email.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("email"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("email", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }
        if !self.password.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("password"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("password", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }

        if !valid {
            return Err(Error::BadRequest(ApiErrorResponse {
                status: false,
                errors,
                request_id: None,
            }));
        }

        Ok(errors)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub user: UserModel,
    pub authenticated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileCommand {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordCommand {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminProfileResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<UserModel> for AdminProfileResponse {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
