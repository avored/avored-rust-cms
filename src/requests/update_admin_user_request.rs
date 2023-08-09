use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UpdateAdminUserRequest {
    #[validate(length(min = 1, message = "The full name is a required field."))]
    pub full_name: String
}
