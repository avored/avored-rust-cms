use crate::error::{Error, Result};
use crate::models::BaseModel;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};


/// Represents a password reset request model
#[derive(Serialize, Debug, Deserialize, Clone, Default, PartialEq)]
pub struct PasswordResetModel {

    /// ID of the password reset model
    pub id: String,

    /// Email associated with the password reset model
    pub email: String,

    /// Token for the password reset
    pub token: String,

    /// Creation date and time of the password reset 
    pub status: PasswordResetTokenStatus,

    /// Creation date and time of the password reset model
    pub created_at: Datetime,
}

/// Represents the view model for forgot password functionality
#[derive(Serialize, Default)]
pub struct ForgotPasswordViewModel {
    /// URL link for the password reset
    pub link: String,
}


/// Represents the status of a password reset token
#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum PasswordResetTokenStatus {

    /// Indicates that the password reset token is currently active
    Active,
    #[default]

    /// Indicates that the password reset token has expired
    Expire,
}

impl TryFrom<Object> for PasswordResetModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<Self> {
        let id = val.get("id").get_id()?;
        let email = val.get("email").get_string()?;
        let token = val.get("token").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let status = match val.get("status").get_string()?.as_str() {
            "Active" => PasswordResetTokenStatus::Active,
            "Expire" => PasswordResetTokenStatus::Active,
            _ => PasswordResetTokenStatus::Expire,
        };

        Ok(Self {
            id,
            email,
            token,
            status,
            created_at,
        })
    }
}


/// Represents a model for creating a password reset request
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatablePasswordResetModel {

    /// Email associated with the password reset request
    pub email: String,

    /// Token for the password reset request
    pub token: String,
}
