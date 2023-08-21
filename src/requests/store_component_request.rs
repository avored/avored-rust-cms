use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize, Debug, Clone, Validate)]
pub struct StoreComponentRequest {
    #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,
    #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,

    pub fields: Vec<FieldType>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct FieldType {
    field_type: String,
    name: String,
    identifier: String
}