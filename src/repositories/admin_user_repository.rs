use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::admin_user_model::{AdminUserModel, CreatableAdminUserModel, UpdatableAdminUserModel};
use crate::models::ModelCount;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};
use crate::models::token_claim_model::LoggedInUser;
use crate::PER_PAGE;

use super::into_iter_objects;

#[derive(Clone)]
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
        let sql = "SELECT *, ->admin_user_role->roles.* as roles FROM admin_users WHERE email=$data;";
        let data: BTreeMap<String, Value> = [("data".into(), email.into())].into();

        let responses = datastore.execute(sql, database_session, Some(data)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
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
        let sql = "SELECT *, ->admin_user_role->roles.* as roles FROM type::thing($table, $id);";
        let vars = BTreeMap::from([
            ("table".into(), "admin_users".into()),
            ("id".into(), id.into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let admin_user_model: Result<AdminUserModel> = result_object?.try_into();

        admin_user_model
    }

    pub async fn create_admin_user(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_admin_user_model: CreatableAdminUserModel,
    ) -> Result<AdminUserModel> {
        let sql = "CREATE admin_users CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("full_name".into(), creatable_admin_user_model.full_name.into(),),
            ("email".into(), creatable_admin_user_model.email.into()),
            ("password".into(), creatable_admin_user_model.password.into(),),
            ("profile_image".into(), creatable_admin_user_model.profile_image.into(),),
            ("is_super_admin".into(), creatable_admin_user_model.is_super_admin.into(),),
            ("created_by".into(), creatable_admin_user_model.logged_in_username.clone().into(),),
            ("updated_by".into(), creatable_admin_user_model.logged_in_username.into(),),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
            .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(ress)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
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
            None => Err(Error::Generic("no record found".to_string())),
        };
        let admin_user_model: Result<AdminUserModel> = result_object?.try_into();

        admin_user_model
    }

    // pub async fn delete_admin_user(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     role_id: String,
    // ) -> Result<bool> {
    //     let sql = "
    //         DELETE type::thing($table, $id);";
    //
    //     let vars: BTreeMap<String, Value> = [
    //         ("id".into(), role_id.into()),
    //         ("table".into(), "admin_users".into()),
    //     ]
    //     .into();
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //     let response = responses.into_iter().next().map(|rp| rp.result).transpose();
    //     if response.is_ok() {
    //         return Ok(true);
    //     }
    //
    //     Ok(false)
    // }

    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM admin_users GROUP ALL;";
        let responses = datastore.execute(sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let admin_user_count: Result<ModelCount> = result_object?.try_into();

        admin_user_count
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
    ) -> Result<Vec<AdminUserModel>> {

        let sql = "SELECT *, ->admin_user_role->roles.* as roles FROM admin_users LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut admin_user_list: Vec<AdminUserModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let admin_user_object = object?;

            let admin_user_model: Result<AdminUserModel> = admin_user_object.try_into();
            admin_user_list.push(admin_user_model?);
        }

        Ok(admin_user_list)
    }

    pub async fn attach_admin_user_with_role(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        admin_user_id: String,
        role_id: String,
        logged_in_user: LoggedInUser
    ) -> Result<bool> {
        let sql = format!(
            "RELATE {}:{}->{}->{}:{} CONTENT $attached_data;",
            "admin_users", admin_user_id, "admin_user_role", "roles", role_id
        );

        let attached_data: BTreeMap<String, Value> = [
            ("created_by".into(), logged_in_user.email.clone().into()),
            ("updated_by".into(), logged_in_user.email.into()),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
            .into();

        let vars: BTreeMap<String, Value> = [("attached_data".into(), attached_data.into())].into();

        let responses = datastore
            .execute(sql.as_str(), database_session, Some(vars))
            .await?;
        println!("RESPONSE ATTACHED: {responses:?}");

        let response = responses.into_iter().next().map(|rp| rp.result).transpose();
        if response.is_ok() {
            return Ok(true);
        }

        Ok(true)
    }

    pub async fn detach_admin_user_with_role(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        admin_user_id: String,
        role_id: String
    ) -> Result<bool> {
        let sql = format!(
            "DELETE {}:{}->{} WHERE {}:{};",
            "admin_users", admin_user_id, "admin_user_role", "roles", role_id
        );

        let responses = datastore
            .execute(sql.as_str(), database_session, None)
            .await?;
        println!("RESPONSE DETACHED: {responses:?}");

        let response = responses.into_iter().next().map(|rp| rp.result).transpose();
        if response.is_ok() {
            return Ok(true);
        }

        Ok(true)
    }
}
