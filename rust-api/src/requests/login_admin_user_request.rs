use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct LoginAdminUserRequest {
    #[validate(email(message = "The email field must be a valid email address."))]
    pub email: String,
    #[validate(length(min = 150, message = "minimum 150 character is required"))]
    pub password: String,
}
