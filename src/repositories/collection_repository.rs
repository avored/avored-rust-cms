use super::into_iter_objects;
use crate::error::{Error, Result};
use crate::models::collection_model::{CollectionModel, CreatableCollection, UpdatableCollection};
use crate::models::ModelCount;
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};


/// collection repository
#[derive(Clone)]
pub struct CollectionRepository {}

impl CollectionRepository {

    /// all collection
    pub(crate) async fn all_collection(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<Vec<CollectionModel>> {
        let sql = "SELECT * FROM collections";

        let responses = datastore.execute(sql, database_session, None).await?;

        let mut collection_list: Vec<CollectionModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let model_object = object?;

            let model_model: Result<CollectionModel> = model_object.try_into();
            collection_list.push(model_model?);
        }
        Ok(collection_list)
    }
}

impl Default for CollectionRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionRepository {

    /// new instance
    pub const fn new() -> Self {
        Self {}
    }

    // pub async fn get_total_count(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    // ) -> Result<ModelCount> {
    //     let sql = "SELECT count() FROM collections GROUP ALL;";
    //     let responses = datastore.execute(sql, database_session, None).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found".to_string())),
    //     };
    //     let model_count: Result<ModelCount> = result_object?.try_into();
    //
    //     model_count
    // }

    // pub async fn paginate(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     start: i64,
    //     order_column: String,
    //     order_type: String,
    // ) -> Result<Vec<CollectionModel>> {
    //     let sql = format!(
    //         "\
    //         SELECT * \
    //         FROM collections \
    //         ORDER {} {} \
    //         LIMIT $limit \
    //         START $start;\
    //     ",
    //         order_column, order_type
    //     );
    //     let vars = BTreeMap::from([
    //         ("limit".into(), PER_PAGE.into()),
    //         ("start".into(), start.into()),
    //     ]);
    //     let responses = datastore
    //         .execute(&sql, database_session, Some(vars))
    //         .await?;
    //
    //     let mut paginate_models: Vec<CollectionModel> = Vec::new();
    //
    //     for object in into_iter_objects(responses)? {
    //         let object = object?;
    //
    //         let model_model: Result<CollectionModel> = object.try_into();
    //         paginate_models.push(model_model?);
    //     }
    //     Ok(paginate_models)
    // }



    /// find by id
    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        model_id: &str,
    ) -> Result<CollectionModel> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), model_id.into()),
            ("table".into(), "collections".into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_model: Result<CollectionModel> = result_object?.try_into();

        model_model
    }
    //
    // pub async fn find_by_identifier(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     identifier: &str,
    // ) -> Result<CollectionModel> {
    //     let sql = "SELECT * FROM type::table($table) WHERE identifier=$identifier;";
    //     let vars: BTreeMap<String, Value> = [
    //         ("identifier".into(), identifier.into()),
    //         ("table".into(), "collections".into()),
    //     ]
    //         .into();
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found".to_string())),
    //     };
    //     let model_model: Result<CollectionModel> = result_object?.try_into();
    //
    //     model_model
    // }
    //
    // //
    // //
    // pub async fn update_collection_identifier(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     put_model_identifier_model: PutCollectionIdentifierModel
    // ) -> Result<CollectionModel> {
    //     let sql = "UPDATE type::thing($table, $id)
    //                 SET
    //                     identifier = $identifier,
    //                     updated_at = $updated_at,
    //                     updated_by = $updated_by
    //                 ;
    //     ";
    //
    //     let vars: BTreeMap<String, Value> = [
    //         ("identifier".into(), put_model_identifier_model.identifier.into()),
    //         ("table".into(), "collections".into()),
    //         ("updated_at".into(), Datetime::default().into()),
    //         ("updated_by".into(), put_model_identifier_model.logged_in_username.into()),
    //         ("id".into(), put_model_identifier_model.id.into())
    //     ].into();
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found".to_string())),
    //     };
    //     let updated_model: Result<CollectionModel> = result_object?.try_into();
    //
    //     updated_model
    // }
    //


    /// count of identifier
    pub async fn count_of_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        identifier: &str,
    ) -> Result<ModelCount> {
        let sql = "SELECT count(identifier=$identifier) FROM collections GROUP ALL";

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


    /// update collection
    pub async fn update_collection(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_model: UpdatableCollection,
    ) -> Result<CollectionModel> {
        let sql = "UPDATE type::thing($table, $id) MERGE $data";

        let data: BTreeMap<String, Value> = [
            ("name".into(), updatable_model.name.into()),
            ("identifier".into(), updatable_model.identifier.into()),
            (
                "updated_by".into(),
                updatable_model.logged_in_username.clone().into(),
            ),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();

        let vars: BTreeMap<String, Value> = [
            ("data".into(), data.into()),
            ("table".into(), "collections".into()),
            ("id".into(), updatable_model.id.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        let model: Result<CollectionModel> = result_object?.try_into();

        model
    }



    /// create collection
    pub async fn create_collection(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_model: CreatableCollection,
    ) -> Result<CollectionModel> {
        let sql = "CREATE type::table($table) CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("name".into(), creatable_model.name.into()),
            ("identifier".into(), creatable_model.identifier.into()),
            (
                "created_by".into(),
                creatable_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                creatable_model.logged_in_username.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();

        let vars: BTreeMap<String, Value> = [
            ("data".into(), data.into()),
            ("table".into(), "collections".into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        let model: Result<CollectionModel> = result_object?.try_into();

        model
    }
}
