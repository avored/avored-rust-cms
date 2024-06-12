use serde::Deserialize;

use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatePageRequest {
    // #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,

    // #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,

    pub components_content: Vec<UpdatableComponentContentRequest>,
}


#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableComponentContentRequest {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub fields: Vec<UpdatableComponentFieldContentRequest>
}


#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableComponentFieldContentRequest {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_content: String,
}

impl UpdatePageRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.name.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: String::from("Name is a required field")
            };

            errors.push(error_message);
        }

        if self.identifier.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: String::from("Identifier is a required field")
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
