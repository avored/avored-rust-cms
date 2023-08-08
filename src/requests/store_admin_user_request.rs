use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct StoreAdminUserRequest {
    #[validate(length(min = 1, message = "The full name is a required field."))]
    pub full_name: String,
    #[validate(email(message = "The email field must be a valid email address."))]
    pub email: String,
    #[validate(length(min = 1, message = "The password is a required field."))]
    pub password: String,
    #[validate(must_match (other = "password", message = "The confirmation password should match with password"))]
    pub confirmation_password: String,
}
