use serde::{Deserialize, Serialize};
use crate::models::admin_user_model::AdminUserModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub admin_user_model: AdminUserModel,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggedInUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub demo_data_status: bool,
    pub admin_user_model: AdminUserModel
}