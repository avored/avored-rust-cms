use chrono::NaiveDateTime;
use sea_orm::{ EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, DeleteResult, PaginatorTrait, LoaderTrait};
use serde_derive::Serialize;
use uuid::Uuid;
use entity::{admin_users, roles, admin_users_roles};

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

#[derive(Serialize)]
pub struct AdminUsersPaginate {
    pub results : Vec<admin_users::Model>,
    pub no_of_pages: u64,
    pub current_page: u64
}

#[derive(Serialize)]
pub struct RoleSturct {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String
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
    pub roles: RoleSturct
}

#[derive(Serialize)] 
pub struct AdminUsersTestResponse {
    pub results: Vec<AdminUserResponse>,
}

pub struct AdminUserRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl AdminUserRepository {
    pub fn new(db: sea_orm::DatabaseConnection ) -> AdminUserRepository {
        AdminUserRepository { db }
    }

    pub async fn paginate(&self, per_page: u64, current_page : u64) -> AdminUsersPaginate {
        // let admin_user_test  = admin_users::Entity::find().paginate(&self.db, per_page);
        let admin_users_all  = admin_users::Entity::find().paginate(&self.db, per_page).fetch_page(current_page).await.unwrap();

        let admin_user_list_test = admin_users_all.load_many_to_many(roles::Entity, admin_users_roles::Entity, &self.db).await.unwrap();



        println!("{:?}", admin_user_list_test);
        // let admin_users_pages  = admin_users::Entity::find()
        //     .paginate(&self.db, per_page);
        let admin_users_pages  = admin_users::Entity::find().inner_join(roles::Entity).paginate(&self.db, per_page);

        let admin_user_list = admin_users_pages.fetch_page(current_page).await.unwrap();
        let no_of_pages = admin_users_pages.num_pages().await.unwrap();

        AdminUsersPaginate {
            results: admin_user_list,
            no_of_pages,
            current_page
        }
    }

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
            updated_by: Set(updated_by_admin_user_email)
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
