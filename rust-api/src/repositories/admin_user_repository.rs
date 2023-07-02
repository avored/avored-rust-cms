use chrono::NaiveDateTime;
use sea_orm::{ EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, DeleteResult};
// use serde::{Serialize};
// use serde_derive::Deserialize;
use uuid::{Uuid};
use entity::admin_users;
// use crate::responses::admin_user_response::AdminUserResponse;

#[derive(Clone)]
pub struct AdminUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String
}

// impl Into<AdminUserResponse> for AdminUser {
//     fn into(self) -> AdminUserResponse {
//         AdminUserResponse {
//             id: self.id,
//             name: self.name,
//             email: self.email,
//             created_at: self.created_at,
//             updated_at: self.updated_at,
//             created_by: self.created_by,
//             updated_by: self.updated_by
//         }
//     }
// }


pub struct AdminUserRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl AdminUserRepository {
    pub fn new(db: sea_orm::DatabaseConnection ) -> AdminUserRepository {
        AdminUserRepository { db }
    }

    // pub fn all(&self) -> Vec<AdminUser> {
    //     let conn = &mut self.db.get().unwrap();

    //     admin_users
    //         .load(conn)
    //         .expect("Error loading admin_users")
    // }

    pub async fn paginate(&self, per_page: i64, offset : i64) -> Vec<admin_users::Model> {
        let db = &self.db;

        let admin_users: Vec<admin_users::Model> = admin_users::Entity::find().all(db).await.expect("error which fetch");

        admin_users

        // admin_users
        //     .offset(offset)
        //     .limit(per_page)
        //     .load(conn)
        //     .expect("Error loading admin_users")
    }

    // pub fn count(&self) -> i64 {
    //     todo!()
    //     // let conn = &mut self.db.get().unwrap();

    //     // admin_users
    //     //     .count()
    //     //     .get_result(conn)
    //     //     .expect("Error while doing a count on admin_users")
    // }

    pub async fn create(
        &self, 
        name: String,
        email: String,
        password: String,
        logged_in_user_email: String
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
            ..Default::default()
        };

        admin_user_model.insert(&self.db).await.unwrap()

    }
  
    pub async fn find_by_email(&self, admin_user_email: String) -> entity::admin_users::Model {
        let expect_message = format!("Error loading admin_users by email: {}", &admin_user_email);
        
        admin_users::Entity::find()
            .filter(admin_users::Column::Email.eq(admin_user_email))
            .one(&self.db).await.expect(&expect_message)
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
    pub async fn update_by_uuid(&self, admin_user_uuid: Uuid, admin_user_email: String) -> entity::admin_users::Model {

        let mut admin_user_model: admin_users::ActiveModel = self.find_by_uuid(admin_user_uuid).await.into();
        admin_user_model.email = Set(admin_user_email);

        admin_user_model.update(&self.db).await.expect("error")
    }

    pub async fn delete_by_uuid(&self, admin_user_uuid: Uuid) -> DeleteResult {
        admin_users::Entity::delete_by_id(admin_user_uuid).exec(&self.db).await.unwrap()
    }
}
