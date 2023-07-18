use crate::requests::update_role_request::UpdateRoleRequest;
use crate::{
    requests::create_role_request::CreateRoleRequest,
    responses::roless_paginate_response::RolessPaginateResponse,
};
use chrono::Utc;
use entity::roles;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait, PaginatorTrait};
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

        if create_role_data.description.is_none() {
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

    pub async fn find_by_uuid(
        &self,
        connection: sea_orm::DatabaseConnection,
        role_uuid: Uuid,
    ) -> entity::roles::Model {
        let expect_message = format!("Error loading roles by uuid: {}", &role_uuid);

        roles::Entity::find_by_id(role_uuid)
            .one(&connection)
            .await
            .expect("error while finding the roles by uuid")
            .ok_or(expect_message)
            .expect("Cannot find roles with uuid")
    }

    pub async fn update_by_uuid(
        &self,
        connection: sea_orm::DatabaseConnection,
        role_uuid: Uuid,
        payload: UpdateRoleRequest,
        logged_in_user_email: String,
    ) -> entity::roles::Model {
        let expect_message = format!("Error loading roles by uuid: {}", &role_uuid);

        let role_model = roles::Entity::find_by_id(role_uuid)
            .one(&connection)
            .await
            .expect("error while finding the admin_users by uuid")
            .ok_or(expect_message)
            .expect("Cannot find admin_users with email");

        let mut active_role_model: roles::ActiveModel = role_model.into();

        active_role_model.name = Set(payload.name);

        let mut desciption: Option<_> = payload.description.clone();

        if payload.description.is_none() {
            desciption = None;
        }

        active_role_model.description = Set(desciption);
        active_role_model.updated_at = Set(Utc::now().naive_utc());
        active_role_model.updated_by = Set(logged_in_user_email);

        active_role_model
            .update(&connection)
            .await
            .expect("Error loading updated roles by uuid")
    }

    pub async fn delete_by_uuid(
        &self,
        connection: sea_orm::DatabaseConnection,
        role_uuid: Uuid,
    ) -> DeleteResult {
        roles::Entity::delete_by_id(role_uuid)
            .exec(&connection)
            .await
            .unwrap()
 
 
    }
    pub async fn all(
        &self,
        connection: sea_orm::DatabaseConnection,
    ) -> Result<Vec<roles::Model>, sea_orm::DbErr>  {
        roles::Entity::find().all(&connection).await
    }
}
