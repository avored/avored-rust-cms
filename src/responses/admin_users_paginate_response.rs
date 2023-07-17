use chrono::NaiveDateTime;
use entity::{admin_users, roles};
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RoleResponseModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize)]
pub struct AdminUserResponseModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
    pub roles: Vec<RoleResponseModel>,
}

#[derive(Serialize)]
pub struct AdminUsersPaginateResponse {
    pub results: Vec<AdminUserResponseModel>, // pub id: Uuid,
                                              // pub name: String,
                                              // pub email: String,
                                              // pub created_at: NaiveDateTime,
                                              // pub updated_at: NaiveDateTime,
                                              // pub created_by: String,
                                              // pub updated_by: String,
}

impl AdminUsersPaginateResponse {
    pub fn transform_into_response(
        admin_users_models: Vec<admin_users::Model>,
        roles: Vec<Vec<roles::Model>>,
    ) -> AdminUsersPaginateResponse {
        let mut admin_users = Vec::new();

        for (admin_user_model, roles) in admin_users_models.into_iter().zip(roles.into_iter()) {
            let mut roles_response_list = Vec::new();

            for role_model in roles {
                let role_response = RoleResponseModel {
                    id: role_model.id,
                    name: role_model.name,
                    description: role_model.description,
                    created_at: role_model.created_at,
                    updated_at: role_model.updated_at,
                    created_by: role_model.created_by,
                    updated_by: role_model.updated_by,
                };

                roles_response_list.push(role_response)
            }

            let admin_user_response = AdminUserResponseModel {
                id: admin_user_model.id,
                name: admin_user_model.name,
                email: admin_user_model.email,
                created_at: admin_user_model.created_at,
                updated_at: admin_user_model.updated_at,
                created_by: admin_user_model.created_by,
                updated_by: admin_user_model.updated_by,
                roles: roles_response_list,
            };
            admin_users.push(admin_user_response)
        }

        AdminUsersPaginateResponse {
            results: admin_users,
        }
    }
}
