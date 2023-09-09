use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::role_model::RoleModel;
use crate::PER_PAGE;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

use super::into_iter_objects;

pub struct RoleRepository {}

impl RoleRepository {
    pub fn new() -> Self {
        RoleRepository {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
    ) -> Result<Vec<RoleModel>> {
        let sql = "SELECT * FROM roles LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut role_list: Vec<RoleModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let role_object = object?;

            let role_model: Result<RoleModel> = role_object.try_into();
            role_list.push(role_model?);
        }
        Ok(role_list)
    }

    // pub async fn find_by_id(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     id: String,
    // ) -> Result<RoleModel> {
    //     let sql = "SELECT * FROM type::thing($table, $id);";
    //     let vars = BTreeMap::from([("table".into(), "roles"), ("id".into(), id.into())]);

    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;

    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //     let role_model: Result<RoleModel> = result_object?.try_into();

    //     role_model
    // }

    // pub async fn update_admin_user(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     updatable_admin_user: UpdatableAdminUserModel,
    // ) -> Result<AdminUserModel> {
    //     let sql = "
    //         UPDATE type::thing($table, $id) MERGE {
    //             full_name: $full_name,
    //             profile_image: $profile_image,
    //             is_super_admin: $is_super_admin,
    //             updated_by: $logged_in_user_name,
    //             updated_at: time::now()
    //         };";

    //     let vars = BTreeMap::from([
    //         ("full_name".into(), updatable_admin_user.full_name.into()),
    //         (
    //             "logged_in_user_name".into(),
    //             updatable_admin_user.logged_in_username.into(),
    //         ),
    //         (
    //             "profile_image".into(),
    //             updatable_admin_user.profile_image.into(),
    //         ),
    //         (
    //             "is_super_admin".into(),
    //             updatable_admin_user.is_super_admin.into(),
    //         ),
    //         ("id".into(), updatable_admin_user.id.into()),
    //         ("table".into(), "admin_users".into()),
    //     ]);

    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;

    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //     let admin_user_model: Result<AdminUserModel> = result_object?.try_into();

    //     admin_user_model
    // }
}
