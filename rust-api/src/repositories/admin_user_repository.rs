use chrono::NaiveDateTime;
use entity::{admin_users, admin_users_roles, roles};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, LoaderTrait, PaginatorTrait,
    QueryFilter, Set,
};
use serde_derive::Serialize;
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

#[derive(Serialize)]
pub struct AdminUsersPaginate {
    pub results: Vec<admin_users::Model>,
    pub no_of_pages: u64,
    pub current_page: u64,
}

#[derive(Serialize)]
pub struct RoleSturct {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
    pub roles: RoleSturct,
}

#[derive(Serialize)]
pub struct AdminUsersTestResponse {
    pub results: Vec<AdminUserResponse>,
}

pub struct AdminUserRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl AdminUserRepository {
    pub fn new(db: sea_orm::DatabaseConnection) -> AdminUserRepository {
        AdminUserRepository { db }
    }

    pub async fn paginate(&self, per_page: u64, current_page: u64) -> AdminUsersPaginateResponse {
        let admin_users_paginate_list = admin_users::Entity::find()
            .paginate(&self.db, per_page)
            .fetch_page(current_page)
            .await
            .unwrap();
        let role_paginate_list = admin_users_paginate_list
            .load_many_to_many(roles::Entity, admin_users_roles::Entity, &self.db)
            .await
            .unwrap();

        AdminUsersPaginateResponse::transform_into_response(
            admin_users_paginate_list,
            role_paginate_list,
        )
    }

    pub async fn create(
        &self,
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

        admin_user_model.insert(&self.db).await.unwrap()
    }

    pub async fn find_by_email(&self, admin_user_email: String) -> entity::admin_users::Model {
        let expect_message = format!("Error loading admin_users by email: {}", &admin_user_email);

        admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(admin_user_email))
            .one(&self.db)
            .await
            .expect(&expect_message)
            .ok_or(expect_message)
            .expect("Cannot find admin_users with email")
    }

    pub async fn find_by_uuid(&self, admin_user_uuid: Uuid) -> entity::admin_users::Model {
        let expect_message = format!("Error loading admin_users by uuid: {}", &admin_user_uuid);

        admin_users::Entity::find_by_id(admin_user_uuid)
            .one(&self.db)
            .await
            .expect("error while finding the admin_users by uuid")
            .ok_or(expect_message)
            .expect("Cannot find admin_users with email")
    }
    pub async fn update_by_uuid(
        &self,
        admin_user_uuid: Uuid,
        admin_user_email: String,
    ) -> entity::admin_users::Model {
        let mut admin_user_model: admin_users::ActiveModel =
            self.find_by_uuid(admin_user_uuid).await.into();
        admin_user_model.email = Set(admin_user_email);

        admin_user_model.update(&self.db).await.expect("error")
    }

    pub async fn delete_by_uuid(&self, admin_user_uuid: Uuid) -> DeleteResult {
        admin_users::Entity::delete_by_id(admin_user_uuid)
            .exec(&self.db)
            .await
            .unwrap()
    }
}
