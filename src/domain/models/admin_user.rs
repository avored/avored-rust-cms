use crate::domain::models::BaseModel;
use crate::error::{Error, Result};
use crate::infra::grpc::admin_user_message::AdminUserMessage;
use pbjson_types::Timestamp;
use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;
use surrealdb::types::Value;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AdminUserModel {
    /// The unique identifier for the admin user.
    pub id: String,

    /// The full name of the admin user.
    pub full_name: String,

    /// The email address of the admin user.
    pub email: String,

    /// The password of the admin user.
    pub password_hash: String,

    /// The profile image URL of the admin user.
    pub profile_image: String,

    /// Indicates whether the admin user has super admin privileges.
    pub is_super_admin: bool,

    /// The date and time when the admin user was created.
    pub created_at: Datetime,

    /// The date and time when the admin user was last updated.
    pub updated_at: Datetime,

    /// The username of the user who created this admin user.
    pub created_by: String,

    /// The username of the user who last updated this admin user.
    pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorableAdminUser {
    pub full_name: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub email: String,
    pub password_hash: String,
    pub logged_in_user: String,
}

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

impl TryFrom<&Value> for AdminUserModel {
    type Error = Error;
    fn try_from(val: &Value) -> Result<Self> {
        let id = val.get_id()?;
        let email = val.get("email").get_string()?;
        let password_hash = val.get("password_hash").get_string()?;

        Ok(Self {
            id,
            email,
            password_hash,
            full_name: val.get("full_name").get_string()?,
            profile_image: val.get("profile_image").get_string()?,
            is_super_admin: val.get("is_super_admin").get_bool()?,
            created_at: val.get("created_at").get_datetime()?,
            updated_at: val.get("updated_at").get_datetime()?,
            created_by: val.get("created_by").get_string()?,
            updated_by: val.get("updated_by").get_string()?,
        })
    }
}

// region: impl try_from AdminUserModel
impl TryFrom<AdminUserModel> for TokenClaims {
    type Error = Error;

    fn try_from(val: AdminUserModel) -> Result<Self> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let claims: Self = Self {
            sub: val.clone().id,
            name: val.clone().full_name,
            email: val.clone().email,
            admin_user_model: val,
            exp,
            iat,
        };

        Ok(claims)
    }
}



impl TryFrom<AdminUserModel> for AdminUserMessage {
    type Error = Error;

    fn try_from(val: AdminUserModel) -> Result<Self> {
        let created_at = Timestamp::from(val.created_at.to_utc());
        let updated_at = Timestamp::from(val.updated_at.to_utc());
        
        let admin_user_message: Self = Self {
            full_name: val.full_name,
            email: val.email,
            profile_image: val.profile_image,
            is_super_admin: val.is_super_admin,
            created_by: val.created_by,
            updated_by: val.updated_by,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        };

        Ok(admin_user_message)
    }
}
