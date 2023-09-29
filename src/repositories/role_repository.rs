use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::role_model::{CreatableRole, RoleModel, UpdatableRoleModel};
use crate::models::ModelCount;
use crate::PER_PAGE;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};

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

    pub async fn create_role(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        createable_role_model: CreatableRole,
    ) -> Result<RoleModel> {
        let sql = "CREATE roles CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("name".into(), createable_role_model.name.into()),
            ("identifier".into(), createable_role_model.identifier.into()),
            (
                "permissions".into(),
                createable_role_model.permissions.into(),
            ),
            (
                "created_by".into(),
                createable_role_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                createable_role_model.logged_in_username.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let role_model: Result<RoleModel> = result_object?.try_into();

        role_model
    }

    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        role_id: String,
    ) -> Result<RoleModel> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), role_id.into()),
            ("table".into(), "roles".into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let role_model: Result<RoleModel> = result_object?.try_into();

        role_model
    }

    pub async fn update_role(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_admin_user: UpdatableRoleModel,
    ) -> Result<RoleModel> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                name: $name,
                identifier: $identifier,
                updated_by: $logged_in_user_name,
                updated_at: time::now(),
                permissions: $permissions
            };";

        // let mut role_ids = Vec::new();
        // role_ids.push("test1");
        // role_ids.push("test2");

        let vars = BTreeMap::from([
            ("name".into(), updatable_admin_user.name.into()),
            ("identifier".into(), updatable_admin_user.identifier.into()),
            (
                "permissions".into(),
                updatable_admin_user.permissions.into(),
            ),
            (
                "logged_in_user_name".into(),
                updatable_admin_user.logged_in_username.into(),
            ),
            // ("permissions".into(), role_ids.into()),
            ("id".into(), updatable_admin_user.id.into()),
            ("table".into(), "roles".into()),
        ]);

        // let vars2: BTreeMap<String, Value> = [

        // ("table".into(), "admin_users".into()),
        // ]
        // .into();

        // println!("{:?}", vars2);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        // println!("UPDATE Role: {:?}", responses);

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let role_model: Result<RoleModel> = result_object?.try_into();

        role_model
    }

    pub async fn delete_role(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        role_id: String,
    ) -> Result<bool> {
        let sql = "
            DELETE type::thing($table, $id);";

        let vars: BTreeMap<String, Value> = [
            ("id".into(), role_id.into()),
            ("table".into(), "roles".into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;
        let response = responses.into_iter().next().map(|rp| rp.result).transpose();
        if response.is_ok() {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM roles GROUP ALL;";
        let responses = datastore.execute(sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }
}
