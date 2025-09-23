use super::BaseModel;
use crate::api::proto::admin_user::AdminUserModel as GrpcAdminUserModel;
use crate::api::proto::admin_user::RoleModel as GrpcRoleModel;
use crate::error::{Error, Result};
use crate::models::role_model::RoleModel;
use crate::models::token_claim_model::TokenClaims;
use crate::services::admin_user_service::AdminUserService;
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use surrealdb::sql::{Datetime, Object, Value};

/// Represents an admin user model in the system.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AdminUserModel {

    /// The unique identifier for the admin user.
    pub id: String,

    /// The full name of the admin user.
    pub full_name: String,

    /// The email address of the admin user.
    pub email: String,

    /// The password of the admin user.
    pub password: String,

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

    /// The roles assigned to the admin user.
    pub roles: Vec<RoleModel>,
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

impl TryFrom<AdminUserModel> for GrpcAdminUserModel {
    type Error = Error;

    fn try_from(val: AdminUserModel) -> Result<Self> {
        let chrono_utc_created_at = val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at = val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let mut grpc_roles: Vec<GrpcRoleModel> = vec![];

        for role in val.roles {
            let grpc_role: GrpcRoleModel = role.try_into()?;
            grpc_roles.push(grpc_role);
        }

        let model: Self = Self {
            id: val.id,
            full_name: val.full_name,
            email: val.email,
            profile_image: val.profile_image,
            is_super_admin: val.is_super_admin,
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
            roles: grpc_roles,
        };

        Ok(model)
    }
}

impl TryFrom<Object> for AdminUserModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<Self> {
        let id = val.get("id").get_id()?;
        let full_name = val.get("full_name").get_string()?;
        let email = val.get("email").get_string()?;
        let password = val.get("password").get_string()?;
        let mut profile_image = val.get("profile_image").get_string()?;
        if profile_image.is_empty() {
            profile_image = String::from("https://place-hold.it/250x250");
        } else {
            //@todo fix this
            profile_image = format!("http://localhost:50051/public/{profile_image}");
        }

        let is_super_admin = val.get("is_super_admin").get_bool()?;

        let roles = match val.get("roles") {
            Some(val) => match val.clone() {
                Value::Array(v) => {
                    let mut arr = Vec::new();

                    for array in v {
                        let object = match array.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        let role_model: RoleModel = object.try_into()?;

                        arr.push(role_model);
                    }
                    arr
                }
                _ => Vec::new(),
            },
            None => Vec::new(),
        };

        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(Self {
            id,
            full_name,
            email,
            password,
            profile_image,
            is_super_admin,
            created_at,
            updated_at,
            created_by,
            updated_by,
            roles,
        })
    }
}

// endregion: impl try_from AdminUserModel


/// Represents a model for creating an admin user.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableAdminUserModel {
    /// The full name of the admin user.
    pub full_name: String,

    /// The email address of the admin user.
    pub email: String,

    /// The password of the admin user.
    pub password: String,

    /// The profile image URL of the admin user.
    pub profile_image: String,

    /// Indicates whether the admin user has super admin privileges.
    pub is_super_admin: bool,

    /// The username of the user who is creating this admin user.
    pub logged_in_username: String,
    // pub role_ids: Vec<String>,
}

/// Represents a model for updating an admin user.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableAdminUserModel {
    /// The unique identifier for the admin user.
    pub id: String,

    /// The full name of the admin user.
    pub full_name: String,

    /// The email address of the admin user.
    pub profile_image: String,
    
    ///  is the admin user has super admin privileges.
    pub is_super_admin: bool,

    /// The username of the user who is updating this admin user.
    pub logged_in_username: String,

    /// The roles assigned to the admin user.
    pub role_ids: Vec<String>,
}

// /// Represents a paginated response for admin users.
// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct AdminUserPagination {
//     /// The list of admin users in the current page.
//     pub data: Vec<AdminUserModel>,

//     /// The total number of admin users available.
//     pub pagination: Pagination,
// }

/// Extension trait for `AdminUserModel` to check resource access
pub trait AdminUserModelExtension {
    /// Checks if the user has access to a specific resource based on the permission identifier.
    fn check_user_has_resouce_access(
        &self,
        admin_user_service: &AdminUserService,
        permission_identifier: String,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl AdminUserModelExtension for AdminUserModel {
    async fn check_user_has_resouce_access(
        &self,
        admin_user_service: &AdminUserService,
        permission_identifier: String,
    ) -> Result<()> {
        let logged_in_user = self.clone();
        let has_permission_bool = admin_user_service
            .has_permission(logged_in_user, permission_identifier.clone())
            .await?;
        if !has_permission_bool {
            return Err(Error::Unauthorizeed(permission_identifier));
        }

        Ok(())
    }
}
