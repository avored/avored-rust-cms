use crate::error::Result;
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

#[derive(Deserialize, Debug, Clone, Validate, Default)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,

    #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,

    pub permissions: Vec<String>,
}

impl UpdateRoleRequest {
    pub fn _validate_errors(&self) -> Result<ValidationErrors> {
        let validation_error_list = match self.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(errors) => errors,
        };

        // for (field_name, error) in validation_error_list.errors() {
        //     match &error {
        //         ValidationErrorsKind::Field(field_errors) => {
        //             for field_error in field_errors {
        //                 let message = match &field_error.message {
        //                     Some(message) => message,
        //                     None => continue,
        //                 };
        //
        //                 if !message.is_empty() {
        //                     // let key = field_name.clone();
        //                     let validation_key = format!("validation_error_{}", field_name);
        //                     session
        //                         .insert(&validation_key, message)
        //                         .expect("Could not store the validation errors into session.");
        //                 }
        //             }
        //         }
        //         ValidationErrorsKind::Struct(_) => continue,
        //         ValidationErrorsKind::List(_) => continue,
        //     }
        // }

        Ok(validation_error_list)
    }
}
