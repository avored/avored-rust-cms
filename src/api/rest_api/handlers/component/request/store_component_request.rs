use crate::error::Result;
use serde::Deserialize;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct StoreComponentRequest {
    #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,

    #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,

    pub fields: Vec<Field>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Field {
    pub name: String,
    pub identifier: String,
    pub field_type: String,
}

impl StoreComponentRequest {
    // pub fn validate_errors(&self) -> Result<ValidationErrors> {
    //     let validation_error_list = match self.validate() {
    //         Ok(_) => ValidationErrors::new(),
    //         Err(errors) => errors,
    //     };
    //
    //     // for (_field_name, error) in validation_error_list.errors() {
    //     //     match &error {
    //     //         ValidationErrorsKind::Field(field_errors) => {
    //     //             for _field_error in field_errors {
    //     //                 // let message = match &field_error.message {
    //     //                 //     Some(message) => message,
    //     //                 //     None => continue,
    //     //                 // };
    //     //             }
    //     //         }
    //     //         ValidationErrorsKind::Struct(_) => continue,
    //     //         ValidationErrorsKind::List(_) => continue,
    //     //     }
    //     // }
    //
    //     Ok(validation_error_list)
    // }
}
