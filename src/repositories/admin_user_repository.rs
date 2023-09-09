use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::admin_user_model::{AdminUserModel, UpdatableAdminUserModel};
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
    pub async fn update_admin_user(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_admin_user: UpdatableAdminUserModel,
    ) -> Result<AdminUserModel> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                full_name: $full_name,
                profile_image: $profile_image,
                is_super_admin: $is_super_admin,
                updated_by: $logged_in_user_name,
                updated_at: time::now()
            };";

        let vars = BTreeMap::from([
            ("full_name".into(), updatable_admin_user.full_name.into()),
            (
                "logged_in_user_name".into(),
                updatable_admin_user.logged_in_username.into(),
            ),
            (
                "profile_image".into(),
                updatable_admin_user.profile_image.into(),
            ),
            (
                "is_super_admin".into(),
                updatable_admin_user.is_super_admin.into(),
            ),
            ("id".into(), updatable_admin_user.id.into()),
            ("table".into(), "admin_users".into()),
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
