
use diesel::{PgConnection};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{prelude::*};
use chrono::NaiveDateTime;
// use diesel::prelude::*;
use serde::{Serialize};
use serde_derive::Deserialize;
use uuid::Uuid;


use crate::responses::admin_user_response::AdminUserResponse;
use crate::schema::admin_users;
use crate::schema::admin_users::dsl::*;


// pub type Email = String;
// pub type Password = String;

#[derive(Queryable, Selectable, Serialize, Debug, Deserialize, Clone)]
#[diesel(table_name = admin_users)]
pub struct AdminUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String
}

#[derive(Insertable)]
#[diesel(table_name = admin_users)]
pub struct NewAdminUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String
}




impl Into<AdminUserResponse> for AdminUser {
    fn into(self) -> AdminUserResponse {
        AdminUserResponse {
            id: self.id,
            name: self.name,
            email: self.email,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by: self.created_by,
            updated_by: self.updated_by
        }
    }
}


pub struct AdminUserRepository {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

impl AdminUserRepository {
    pub fn new(db: Pool<ConnectionManager<PgConnection>> ) -> AdminUserRepository {
        AdminUserRepository { db }
    }

    // pub fn all(&self) -> Vec<AdminUser> {
    //     let conn = &mut self.db.get().unwrap();

    //     admin_users
    //         .load(conn)
    //         .expect("Error loading admin_users")
    // }

    pub fn paginate(&self, per_page: i64, offset : i64) -> Vec<AdminUser> {
        let conn = &mut self.db.get().unwrap();

        admin_users
            .offset(offset)
            .limit(per_page)
            .load(conn)
            .expect("Error loading admin_users")
    }

    pub fn count(&self) -> i64 {
        let conn = &mut self.db.get().unwrap();

        admin_users
            .count()
            .get_result(conn)
            .expect("Error while doing a count on admin_users")
    }

    pub fn create(&self, create_admin_user_model: NewAdminUser) -> AdminUser {
        let conn = &mut self.db.get().unwrap();

        diesel::insert_into(admin_users)
            .values(&create_admin_user_model)
            .get_result::<AdminUser>(conn)
            .expect("Error creating new admin user record")
    }
  
    pub fn find_by_email(&self, admin_user_email: String) -> AdminUser {
        let conn = &mut self.db.get().unwrap();
        let expect_message = format!("Error loading admin_users by email: {}", &admin_user_email);

        admin_users
            .filter(email.eq(admin_user_email))
            .first::<AdminUser>(conn)
            .expect(&expect_message)
    }
    pub fn find_by_uuid(&self, admin_user_uuid: Uuid) -> AdminUser {
        let conn = &mut self.db.get().unwrap();
        let expect_message = format!("Error loading admin_users by id: {}", &admin_user_uuid);

        admin_users
            .filter(id.eq(admin_user_uuid))
            .first::<AdminUser>(conn)
            .expect(&expect_message)
    }
    pub fn update_by_uuid(&self, admin_user_uuid: Uuid, admin_user_email: String) -> AdminUser {
        let conn = &mut self.db.get().unwrap();
        let expect_message = format!("Error updating admin_users by id: {}", &admin_user_uuid);
        
        let current = chrono::offset::Utc::now().naive_utc();
        
        diesel::update(admin_users)
            .filter(id.eq(admin_user_uuid))
            .set((email.eq(admin_user_email), updated_at.eq(current)))
            .execute(conn)
            .expect(&expect_message);
    
        let expect_message = format!("Error loading updated admin_users by id: {}", &admin_user_uuid);

        admin_users
            .filter(id.eq(admin_user_uuid))
            .first::<AdminUser>(conn)
            .expect(&expect_message)
    }

    pub fn delete_by_uuid(&self, admin_user_uuid: Uuid) -> usize {
        let conn = &mut self.db.get().unwrap();
        let expect_message = format!("Error delete admin_users by id: {}", &admin_user_uuid);
        
        diesel::delete(admin_users)
            .filter(id.eq(admin_user_uuid))
            .execute(conn)
            .expect(&expect_message)
    }
}
