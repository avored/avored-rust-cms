use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthenticateAdminUserRequest {
    #[validate(email(message = "The email field must be a valid email address."))]
    pub email: String,
    #[validate(length(min = 1, message = "The password is a required field."))]
    pub password: String,
}
