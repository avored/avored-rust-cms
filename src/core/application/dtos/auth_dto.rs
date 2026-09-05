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
