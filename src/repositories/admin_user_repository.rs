use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::admin_user_model::AdminUserModel;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

use super::into_iter_objects;
pub struct AdminUserRepository {}

impl AdminUserRepository {
    pub fn new() -> Self {
        AdminUserRepository {}
    }

    pub async fn find_by_email(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        email: String,
    ) -> Result<AdminUserModel> {
        let sql = "SELECT * FROM admin_users WHERE $data;";
        let data: BTreeMap<String, Value> = [("email".into(), email.into())].into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let admin_user_model: Result<AdminUserModel> = result_object?.try_into();

        admin_user_model
    }
    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: String,
    ) -> Result<AdminUserModel> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        let vars = BTreeMap::from([
            ("table".into(), "admin_users".into()),
            ("id".into(), id.into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let admin_user_model: Result<AdminUserModel> = result_object?.try_into();

        admin_user_model
    }
}
