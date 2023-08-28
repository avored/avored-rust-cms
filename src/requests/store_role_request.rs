use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct StoreRoleRequest {
    #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,
    #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,
}
