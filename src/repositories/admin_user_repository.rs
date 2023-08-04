use surrealdb::dbs::{Session, Response};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Array, Object};

use crate::models::W;
use crate::error::Result;
use crate::models::admin_user_model::AdminUser;

pub struct AdminUserRepository {}

impl AdminUserRepository {
    pub fn new() -> AdminUserRepository {
        AdminUserRepository {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session
    ) -> Result<Vec<AdminUser>> {

        let sql = "SELECT * FROM admin_users;";

        let responses = match datastore.execute(sql, &database_session, None, false).await {
            Ok(response) => response,
            Err(_) => {
                // todo improve this error
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("error while retriving the first response");

        let result = response.result.expect("first result comes with error");

        let array: Array = W(result).try_into().expect("sdfds");
        let objects: Result<Vec<Object>> =
            array.into_iter().map(|value| W(value).try_into()).collect();
        let objects = match objects {
            Ok(obj) => obj,
            Err(_) => panic!("no data"),
        };

        let test: Result<Vec<AdminUser>> = objects.into_iter().map(|o| o.try_into()).collect();

        let admin_users = match test {
            Ok(data) => data,
            Err(_) => panic!("some errror"),
        };

        Ok(admin_users)
    }
}
