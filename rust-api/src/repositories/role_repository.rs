use crate::{
    requests::create_role_request::CreateRoleRequest,
    responses::roless_paginate_response::RolessPaginateResponse,
};
use chrono::Utc;
use entity::roles;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait};
use uuid::Uuid;

pub struct RoleRepository {}

impl RoleRepository {
    pub fn new() -> RoleRepository {
        RoleRepository {}
    }

    pub async fn paginate(
        &self,
        conn: sea_orm::DatabaseConnection,
        per_page: u64,
        current_page: u64,
    ) -> RolessPaginateResponse {
        let roles_paginate_list = roles::Entity::find()
            .paginate(&conn, per_page)
            .fetch_page(current_page)
            .await
            .unwrap();

        RolessPaginateResponse::transform_into_response(roles_paginate_list)
    }

    pub async fn create(
        &self,
        connection: sea_orm::DatabaseConnection,
        create_role_data: CreateRoleRequest,
        logged_in_user_email: String,
    ) -> entity::roles::Model {
        let current = Utc::now().naive_utc();
        let updated_by_admin_user_email = logged_in_user_email.clone();

        let mut desciption = create_role_data.description.clone();

        if !create_role_data.description.is_none() {
            desciption = None;
        }

        let role_model = roles::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(create_role_data.name),
            description: Set(desciption),
            created_at: Set(current),
            updated_at: Set(current),
            created_by: Set(logged_in_user_email),
            updated_by: Set(updated_by_admin_user_email),
        };

        role_model.insert(&connection).await.unwrap()
    }
}
