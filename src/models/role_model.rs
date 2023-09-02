use crate::error::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct RoleModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl TryFrom<Object> for RoleModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<RoleModel> {
        
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
     
        let name = match val.get("name") {
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

        let identifier = match val.get("identifier") {
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

        Ok(RoleModel {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}



impl RoleModel {
    pub fn empty() -> Self {
        RoleModel {
            id: String::from(""),
            name: String::from(""),
            identifier: String::from(""),
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            created_by: String::from(""),
            updated_by: String::from(""),
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableRole {
    pub name: String,
    pub identifier: String,
    pub logged_in_user_email: String,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableRole {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub logged_in_user_email: String,
}
