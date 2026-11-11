use serde::{Deserialize, Serialize};

use crate::core::application::dtos::pagination_dto::PaginatedResponse;
use crate::core::domain::entities::{
    error_message::{ApiErrorResponse, ErrorMessageResponse},
    EmailTemplateModel,
};
use crate::core::domain::extensions::string_extension::StringExtension;
use crate::error::Result;

pub type PaginateEmailTemplateCommand =
    crate::core::application::dtos::pagination_dto::PaginationQuery;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateEmailTemplateCommand {
    pub name: String,
    pub subject: String,
    pub body_html: Option<String>,
    pub body_plain: Option<String>,
}

impl CreateEmailTemplateCommand {
    pub async fn validate(&self, _locale: &str) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if self.name.trim().is_empty() || !self.name.is_required()? {
            errors.push(ErrorMessageResponse {
                key: "name".to_string(),
                message: "Name is required.".to_string(),
            });
            valid = false;
        }

        if self.subject.trim().is_empty() || !self.subject.is_required()? {
            errors.push(ErrorMessageResponse {
                key: "subject".to_string(),
                message: "Subject is required.".to_string(),
            });
            valid = false;
        }

        if !valid {
            return Err(crate::error::Error::BadRequest(ApiErrorResponse {
                status: false,
                errors,
                request_id: None,
            }));
        }

        Ok(errors)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEmailTemplateCommand {
    pub name: String,
    pub subject: String,
    pub body_html: Option<String>,
    pub body_plain: Option<String>,
}

impl UpdateEmailTemplateCommand {
    pub async fn validate(&self, _locale: &str) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if self.name.trim().is_empty() || !self.name.is_required()? {
            errors.push(ErrorMessageResponse {
                key: "name".to_string(),
                message: "Name is required.".to_string(),
            });
            valid = false;
        }

        if self.subject.trim().is_empty() || !self.subject.is_required()? {
            errors.push(ErrorMessageResponse {
                key: "subject".to_string(),
                message: "Subject is required.".to_string(),
            });
            valid = false;
        }

        if !valid {
            return Err(crate::error::Error::BadRequest(ApiErrorResponse {
                status: false,
                errors,
                request_id: None,
            }));
        }

        Ok(errors)
    }
}

pub type EmailTemplatePaginationResponse = PaginatedResponse<EmailTemplateModel>;
