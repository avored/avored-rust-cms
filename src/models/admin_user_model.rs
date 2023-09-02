use serde::Serialize;
use serde_derive::Deserialize;
use surrealdb::sql::{Datetime, Object, Value};

use crate::{error::{Error, Result}, PER_PAGE};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AdminUser {
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
}

impl AdminUser {
    pub fn empty_admin_user() -> Self {
        AdminUser {
            id: String::from(""),
            full_name: String::from(""),
            email: String::from(""),
            password: String::from(""),
            profile_image: String::from(""),
            is_super_admin: false,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            created_by: String::from(""),
            updated_by: String::from(""),
        }
    }
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AdminUserPaginate {
    pub count: i64,
    pub per_page: i64,
    pub from: i64,
    pub to: i64,
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub next_page: i64,
    pub previous_page: i64
}




impl AdminUserPaginate {
    pub fn empty_admin_user_paginate() -> Self {
        AdminUserPaginate {
            count: 0,
            per_page: PER_PAGE,
            from: 0,
            to: 0,
            has_previous_page: false,
            has_next_page: false,
            next_page: 0,
            previous_page: 0
        }
    }
}


impl TryFrom<Object> for AdminUserPaginate {
    type Error = Error;
    fn try_from(val: Object) -> Result<AdminUserPaginate> {
        let count = match val.get("count") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Number(v) => {
						v.as_int()
					},
					_ => i64::default()
				};
				value
			
            },
            None => i64::default(),
        };
        let mut admin_user_paginate = AdminUserPaginate::empty_admin_user_paginate();
        admin_user_paginate.count = count;

        Ok(admin_user_paginate)
    }
}

impl TryFrom<Object> for AdminUser {
    type Error = Error;
    fn try_from(val: Object) -> Result<AdminUser> {
        let id = match val.get("id") {
            Some(val) => { 
                let value  = match val.clone() {
					Value::Thing(v) => {
						let id = v.id;
                        id.to_string()
					},
					_ => String::from("")
				};
				value
            },
            None => String::from(""),
        };
     
        let full_name = match val.get("full_name") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Strand(v) => {
						v.as_string()
					},
					_ => String::from("")
				};
				value
			},
			None => String::from(""),
		};

        let email = match val.get("email") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Strand(v) => {
						v.as_string()
					},
					_ => String::from("")
				};
				value
			},
			None => String::from(""),
		};

        let password = match val.get("password") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Strand(v) => {
						v.as_string()
					},
					_ => String::from("")
				};
				value
			},
			None => String::from(""),
		};

        let profile_image = match val.get("profile_image") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Strand(v) => {
						v.as_string()
					},
					_ => String::from("")
				};
				value
			},
			None => String::from(""),
		};

        let is_super_admin = match val.get("is_super_admin") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Bool(v) => {
						v
					},
					_ => false
				};
				value
			},
			None => false,
		};
        let created_at = match val.get("created_at") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Datetime(v) => {
						v
					},
					_ => Datetime::default()
				};
				value
			},
			None => Datetime::default(),
		};
        let updated_at = match val.get("updated_at") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Datetime(v) => {
						v
					},
					_ => Datetime::default()
				};
				value
			},
			None => Datetime::default(),
		};

         let created_by = match val.get("created_by") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Strand(v) => {
						v.as_string()
					},
					_ => String::from("")
				};
				value
			},
			None => String::from(""),
		};

         let updated_by = match val.get("updated_by") {
			Some(val) => { 
				let value  = match val.clone() {
					Value::Strand(v) => {
						v.as_string()
					},
					_ => String::from("")
				};
				value
			},
			None => String::from(""),
		};
      
        Ok(AdminUser {
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
        })
    }
}



#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableAdminUser {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableAdminUser {
    pub id: String,
    pub full_name: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
}
