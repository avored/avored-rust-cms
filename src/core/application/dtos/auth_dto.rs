use serde::{Deserialize, Serialize};

use crate::core::domain::entities::UserModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub user: UserModel,
    pub authenticated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileCommand {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordCommand {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminProfileResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<UserModel> for AdminProfileResponse {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
