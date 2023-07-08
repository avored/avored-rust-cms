use chrono::NaiveDateTime;
use entity::{admin_users, admin_users_roles, roles};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, LoaderTrait, PaginatorTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

use crate::responses::admin_users_paginate_response::AdminUsersPaginateResponse;

#[derive(Clone)]
pub struct AdminUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}
pub struct AdminUserRepository {
    // pub db: sea_orm::DatabaseConnection,
}

impl AdminUserRepository {
    pub fn new() -> AdminUserRepository {
        AdminUserRepository {}
    }

    pub async fn paginate(
        &self,
        connection: sea_orm::DatabaseConnection,
        per_page: u64,
        current_page: u64,
    ) -> AdminUsersPaginateResponse {
        let admin_users_paginate_list = admin_users::Entity::find()
            .paginate(&connection, per_page)
            .fetch_page(current_page)
            .await
            .unwrap();
        let role_paginate_list = admin_users_paginate_list
            .load_many_to_many(roles::Entity, admin_users_roles::Entity, &connection)
            .await
            .unwrap();

        AdminUsersPaginateResponse::transform_into_response(
            admin_users_paginate_list,
            role_paginate_list,
        )
    }

    pub async fn create(
        &self,
        connection: sea_orm::DatabaseConnection,
        name: String,
        email: String,
        password: String,
        logged_in_user_email: String,
    ) -> entity::admin_users::Model {
        let current = chrono::offset::Utc::now().naive_utc();
        let updated_by_admin_user_email = logged_in_user_email.clone();

        let admin_user_model = admin_users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            email: Set(email),
            password: Set(password),
            created_at: Set(current),
            updated_at: Set(current),
            created_by: Set(logged_in_user_email),
            updated_by: Set(updated_by_admin_user_email),
        };

        admin_user_model.insert(&connection).await.unwrap()
    }

    pub async fn find_by_email(&self, connection: sea_orm::DatabaseConnection, admin_user_email: String) -> entity::admin_users::Model {
        let expect_message = format!("Error loading admin_users by email: {}", &admin_user_email);

        admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(admin_user_email))
            .one(&connection)
            .await
            .expect(&expect_message)
            .ok_or(expect_message)
            .expect("Cannot find admin_users with email")
    }

    pub async fn find_by_uuid(&self, connection: sea_orm::DatabaseConnection, admin_user_uuid: Uuid) -> entity::admin_users::Model {
        let expect_message = format!("Error loading admin_users by uuid: {}", &admin_user_uuid);

        admin_users::Entity::find_by_id(admin_user_uuid)
            .one(&connection)
            .await
            .expect("error while finding the admin_users by uuid")
            .ok_or(expect_message)
            .expect("Cannot find admin_users with email")
    }
    pub async fn update_by_uuid(
        &self,
        connection: sea_orm::DatabaseConnection, 
        admin_user_uuid: Uuid,
        admin_user_email: String,
    ) -> entity::admin_users::Model {
        let expect_message = format!("Error loading admin_users by uuid: {}", &admin_user_uuid);

        let admin_user_model = admin_users::Entity::find_by_id(admin_user_uuid)
            .one(&connection)
            .await
            .expect("error while finding the admin_users by uuid")
            .ok_or(expect_message)
            .expect("Cannot find admin_users with email");

        let mut active_admin_user_model: admin_users::ActiveModel = admin_user_model.into();

        active_admin_user_model.email = Set(admin_user_email);

        active_admin_user_model.update(&connection).await.expect("error")
    }

    pub async fn delete_by_uuid(&self, connection: sea_orm::DatabaseConnection, admin_user_uuid: Uuid) -> DeleteResult {
        admin_users::Entity::delete_by_id(admin_user_uuid)
            .exec(&connection)
            .await
            .unwrap()
    }
}
