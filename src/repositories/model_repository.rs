use std::collections::BTreeMap;

use crate::error::{Error, Result};
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};
use crate::models::model_model::{CreatableModel, ModelModel, PutModelIdentifierModel, UpdatableModelModel};
use crate::models::ModelCount;
use crate::PER_PAGE;
use super::into_iter_objects;

#[derive(Clone)]
pub struct ModelRepository {}

impl ModelRepository {
    pub fn new() -> Self {
        ModelRepository {}
    }


    pub async fn create_model(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_model: CreatableModel,
    ) -> Result<ModelModel> {
        let sql = "CREATE models CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("name".into(), creatable_model.name.into()),
            ("identifier".into(), creatable_model.identifier.into()),
            ("created_by".into(), creatable_model.logged_in_username.clone().into()),
            ("updated_by".into(), creatable_model.logged_in_username.into()),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let created_model: Result<ModelModel> = result_object?.try_into();

        created_model
    }

    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM models GROUP ALL;";
        let responses = datastore.execute(sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
        order_column: String,
        order_type: String
    ) -> Result<Vec<ModelModel>> {
        let sql = format!("\
            SELECT * \
            FROM models \
            ORDER {} {} \
            LIMIT $limit \
            START $start;\
        ", order_column, order_type);
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);
        let responses = datastore.execute(&sql, database_session, Some(vars)).await?;

        let mut paginate_models: Vec<ModelModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let object = object?;

            let model_model: Result<ModelModel> = object.try_into();
            paginate_models.push(model_model?);
        }
        Ok(paginate_models)
    }

    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        model_id: String,
    ) -> Result<ModelModel> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), model_id.into()),
            ("table".into(), "models".into()),
        ]
            .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_model: Result<ModelModel> = result_object?.try_into();

        model_model
    }


    pub async fn update_model_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        put_model_identifier_model: PutModelIdentifierModel
    ) -> Result<ModelModel> {
        let sql = "UPDATE type::thing($table, $id)
                    SET
                        identifier = $identifier,
                        updated_at = $updated_at,
                        updated_by = $updated_by
                    ;
        ";

        let vars: BTreeMap<String, Value> = [
            ("identifier".into(), put_model_identifier_model.identifier.into()),
            ("table".into(), "models".into()),
            ("updated_at".into(), Datetime::default().into()),
            ("updated_by".into(), put_model_identifier_model.logged_in_username.into()),
            ("id".into(), put_model_identifier_model.id.into())
        ].into();
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let updated_model: Result<ModelModel> = result_object?.try_into();

        updated_model
    }

    pub async fn count_of_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        identifier: String
    ) -> Result<ModelCount> {
        let sql = "SELECT count(identifier=$identifier) FROM models GROUP ALL";

        let vars: BTreeMap<String, Value> = [("identifier".into(), identifier.into())].into();
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }

    pub async fn update_model(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_model: UpdatableModelModel,
    ) -> Result<ModelModel> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                name: $name,
                updated_by: $logged_in_user_name,
                updated_at: time::now()
            };";

        let vars = BTreeMap::from([
            ("name".into(), updatable_model.name.into()),
            ("logged_in_user_name".into(), updatable_model.logged_in_username.into()),
            ("id".into(), updatable_model.id.into()),
            ("table".into(), "models".into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_model: Result<ModelModel> = result_object?.try_into();

        model_model
    }

}
