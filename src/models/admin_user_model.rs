use std::time::SystemTime;
use prost_types::Timestamp;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use crate::models::token_claim_model::TokenClaims;
use super::{BaseModel, Pagination};
use crate::api::proto::admin_user::{AdminUserModel as GrpcAdminUserModel};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AdminUserModel {
    pub id: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
    // pub roles: Vec<RoleModel>,
}

// region: impl try_from AdminUserModel
impl TryFrom<AdminUserModel> for TokenClaims {
    type Error = Error;

    fn try_from(val: AdminUserModel) -> Result<TokenClaims> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
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

impl TryFrom<AdminUserModel> for  GrpcAdminUserModel {
    type Error = Error;

    fn try_from(val: AdminUserModel) -> Result<GrpcAdminUserModel> {
        let chrono_utc_created_at= val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at= val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let model: GrpcAdminUserModel = GrpcAdminUserModel {
            id: val.id,
            full_name: val.full_name,
            email: val.email,
            profile_image: val.profile_image,
            is_super_admin: val.is_super_admin,
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
        };

        Ok(model)
    }
}

impl TryFrom<Object> for AdminUserModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<AdminUserModel> {
        let id = val.get("id").get_id()?;
        let full_name = val.get("full_name").get_string()?;
        let email = val.get("email").get_string()?;
        let password = val.get("password").get_string()?;
        let mut profile_image = val.get("profile_image").get_string()?;
        if profile_image.is_empty() {
            profile_image = String::from("https://place-hold.it/250x250");
        } else {
            //@todo fix this
            profile_image = format!("http://localhost:50051/public/{}", profile_image);
        }

        let is_super_admin = val.get("is_super_admin").get_bool()?;

        // let roles = match val.get("roles") {
        //     Some(val) => match val.clone() {
        //         Value::Array(v) => {
        //             let mut arr = Vec::new();
        //
        //             for array in v.into_iter() {
        //                 let object = match array.clone() {
        //                     Value::Object(v) => v,
        //                     _ => surrealdb::sql::Object::default(),
        //                 };
        //
        //                 let role_model: RoleModel = object.try_into()?;
        //
        //                 arr.push(role_model)
        //             }
        //             arr
        //         }
        //         _ => Vec::new(),
        //     },
        //     None => Vec::new(),
        // };

        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(AdminUserModel {
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
            // roles,
        })
    }
}

// endregion: impl try_from AdminUserModel

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableAdminUserModel {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
    // pub role_ids: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableAdminUserModel {
    pub id: String,
    pub full_name: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
    // pub role_ids: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AdminUserPagination {
    pub data: Vec<AdminUserModel>,
    pub pagination: Pagination,
}
