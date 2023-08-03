use serde::Serialize;
use serde_derive::Deserialize;
use surrealdb::sql::{Object, Value};

use crate::error::{Error, Result};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AdminUser {
    pub id: String,
    pub name: String,
}

impl TryFrom<Object> for AdminUser {
    type Error = Error;
    fn try_from(val: Object) -> Result<AdminUser> {
        let id = match val.get("id") {
            Some(val) => val.clone(),
            None => Value::Null,
        };

        let name = match val.get("name") {
            Some(val) => val.clone(),
            None => Value::Null,
        };

        Ok(AdminUser {
            id: id.to_string(),
            name: name.to_string(),
        })
    }
}
