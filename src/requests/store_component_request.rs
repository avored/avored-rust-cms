use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize, Debug, Clone, Validate)]
pub struct StoreComponentRequest {
    #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,
    #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,
    #[validate(length(min = 1, message = "The field name is a required field."))]
    pub field_names: String,
    #[validate(length(min = 1, message = "The field identifier is a required field."))]
    pub field_identifiers: String,
    #[validate(length(min = 1, message = "The field types is a required field."))]
    pub field_types: String,
    // #[validate(length(min = 1, message = "The field types is a required field."))]
}
