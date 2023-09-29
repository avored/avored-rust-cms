use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::field_model::{CreatableFieldModel, FieldModel};
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};

use super::into_iter_objects;

pub struct FieldRepository {}

impl FieldRepository {
    pub fn new() -> Self {
        FieldRepository {}
    }

    // pub async fn paginate(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     start: i64,
    // ) -> Result<Vec<FieldModel>> {
    //     let sql = "SELECT * FROM fields LIMIT $limit START $start;";
    //     let vars = BTreeMap::from([
    //         ("limit".into(), PER_PAGE.into()),
    //         ("start".into(), start.into()),
    //     ]);
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let mut field_list: Vec<FieldModel> = Vec::new();
    //
    //     for object in into_iter_objects(responses)? {
    //         let field_object = object?;
    //
    //         let field_model: Result<FieldModel> = field_object.try_into();
    //         field_list.push(field_model?);
    //     }
    //     Ok(field_list)
    // }

    pub async fn create_field(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        createable_field_model: CreatableFieldModel,
    ) -> Result<FieldModel> {
        let sql = "CREATE fields CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("name".into(), createable_field_model.name.into()),
            (
                "identifier".into(),
                createable_field_model.identifier.into(),
            ),
            (
                "created_by".into(),
                createable_field_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                createable_field_model.logged_in_username.into(),
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
        let field_model: Result<FieldModel> = result_object?.try_into();

        field_model
    }

    // pub async fn find_by_id(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     field_id: String,
    // ) -> Result<FieldModel> {
    //     let sql = "SELECT * FROM type::thing($table, $id);";
    //     let vars: BTreeMap<String, Value> = [
    //         ("id".into(), field_id.into()),
    //         ("table".into(), "fields".into()),
    //     ]
    //     .into();
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //     let field_model: Result<FieldModel> = result_object?.try_into();
    //
    //     field_model
    // }
    //
    // pub async fn update_field(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     updatable_admin_user: UpdatableFieldModel,
    // ) -> Result<FieldModel> {
    //     let sql = "
    //         UPDATE type::thing($table, $id) MERGE {
    //             name: $name,
    //             identifier: $identifier,
    //             updated_by: $logged_in_user_name,
    //             updated_at: time::now()
    //         };";
    //
    //     let vars = BTreeMap::from([
    //         ("name".into(), updatable_admin_user.name.into()),
    //         ("identifier".into(), updatable_admin_user.identifier.into()),
    //         (
    //             "logged_in_user_name".into(),
    //             updatable_admin_user.logged_in_username.into(),
    //         ),
    //         ("id".into(), updatable_admin_user.id.into()),
    //         ("table".into(), "fields".into()),
    //     ]);
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //     let field_model: Result<FieldModel> = result_object?.try_into();
    //
    //     field_model
    // }
    //
    // pub async fn delete_field(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     field_id: String,
    // ) -> Result<bool> {
    //     let sql = "
    //         DELETE type::thing($table, $id);";
    //
    //     let vars: BTreeMap<String, Value> = [
    //         ("id".into(), field_id.into()),
    //         ("table".into(), "fields".into()),
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

    // pub async fn get_total_count(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    // ) -> Result<ModelCount> {
    //     let sql = "SELECT count() FROM fields GROUP ALL;";
    //     let responses = datastore.execute(sql, database_session, None).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //
    //     let total_count = match result_object {
    //         Ok(obj) => obj.try_into(),
    //         Err(_) => Ok(ModelCount::default()),
    //     };
    //
    //     total_count
    // }
}
