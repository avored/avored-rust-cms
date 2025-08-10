use crate::models::admin_user_model::AdminUserModel;
use serde::{Deserialize, Serialize};

/// Represents the claims contained in a JWT token
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TokenClaims {
    
    /// Unique identifier for the user
    pub sub: String,
    /// Name of the user
    /// 
    pub name: String,
    /// Email address of the user
    pub email: String,
    
    /// Indicates whether the user is an admin
    pub admin_user_model: AdminUserModel,

    /// Indicates whether the user has demo data status
    pub iat: usize,

    /// Indicates the expiration time of the token
    pub exp: usize,
}

/// Represents a user that is logged in, containing their details and admin user model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggedInUser {
    /// Unique identifier for the user
    pub id: String,
    /// Name of the user
    pub name: String,
    /// Email address of the user
    pub email: String,

    /// Indicates whether the user is an admin
    pub demo_data_status: bool,
    /// The admin user model associated with the logged-in user
    pub admin_user_model: AdminUserModel,
}
