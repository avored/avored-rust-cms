use super::into_iter_objects;
use crate::error::{Result};
use crate::models::collection_model::{ CollectionModel};
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;

#[derive(Clone)]
pub struct CollectionRepository {}

impl CollectionRepository {
    pub(crate) async fn all_collection(
        &self,
        datastore: &Datastore,
        database_session: &Session
    )  -> Result<Vec<CollectionModel>> {
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

impl CollectionRepository {
    pub fn new() -> Self {
        CollectionRepository {}
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

    // pub async fn find_by_id(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     model_id: String,
    // ) -> Result<CollectionModel> {
    //     let sql = "SELECT * FROM type::thing($table, $id);";
    //     let vars: BTreeMap<String, Value> = [
    //         ("id".into(), model_id.into()),
    //         ("table".into(), "collections".into()),
    //     ]
    //     .into();
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
    // pub async fn count_of_identifier(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     identifier: String
    // ) -> Result<ModelCount> {
    //     let sql = "SELECT count(identifier=$identifier) FROM collections GROUP ALL";
    // 
    //     let vars: BTreeMap<String, Value> = [("identifier".into(), identifier.into())].into();
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
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
    // 
    // pub async fn update_collection(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     updatable_model: UpdatableCollection,
    // ) -> Result<CollectionModel> {
    //     let sql = "UPDATE type::thing($table, $id) CONTENT $data";
    // 
    //     let mut collection_fields: Vec<Value> = vec![];
    // 
    //     for updatable_collection_field in updatable_model.collection_fields {
    //         let data_type_value: Value = match updatable_collection_field.data_type {
    //             CollectionFieldDataType::Text(v) => v.into(),
    //         };
    // 
    //         let field_type_value: Value = match updatable_collection_field.field_type {
    //             CollectionFieldFieldType::Text => "Text".into(),
    // 
    //         };
    // 
    // 
    //         let content_field: BTreeMap<String, Value> = [
    //             ("name".into(), updatable_collection_field.name.into()),
    //             ("identifier".into(), updatable_collection_field.identifier.into()),
    //             ("data_type".into(), data_type_value),
    //             ("field_type".into(), field_type_value),
    //         ]
    //             .into();
    // 
    //         collection_fields.push(content_field.into());
    //     }
    // 
    // 
    //     let data: BTreeMap<String, Value> = [
    //         ("name".into(), updatable_model.name.into()),
    //         ("identifier".into(), updatable_model.identifier.into()),
    //         (
    //             "updated_by".into(),
    //             updatable_model.logged_in_username.clone().into(),
    //         ),
    //         ("created_by".into(), updatable_model.created_by.into()),
    //         ("collection_fields".into(), collection_fields.into()),
    //         ("updated_at".into(), Datetime::default().into()),
    //         ("created_at".into(), updatable_model.created_at.into()),
    //     ]
    //         .into();
    // 
    //     let vars: BTreeMap<String, Value> = [
    //         ("data".into(), data.into()),
    //         ("table".into(), "collections".into()),
    //         ("id".into(), updatable_model.id.into()),
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
    // 
    //     let model: Result<CollectionModel> = result_object?.try_into();
    // 
    //     model
    // }
    // 
    // pub async fn create_collection(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     creatable_model: CreatableCollection,
    // ) -> Result<CollectionModel> {
    //     let sql = "CREATE type::table($table) CONTENT $data";
    // 
    //     let mut collection_fields: Vec<Value> = vec![];
    // 
    //     for creatable_field in creatable_model.collection_fields {
    //         let data_type_value: Value = match creatable_field.data_type {
    //             CollectionFieldDataType::Text(v) => v.into(),
    //         };
    // 
    //         let field_type_value: Value = match creatable_field.field_type {
    //             CollectionFieldFieldType::Text => "Text".into(),
    //         };
    // 
    //         let content_field: BTreeMap<String, Value> = [
    //             ("name".into(), creatable_field.name.into()),
    //             ("identifier".into(), creatable_field.identifier.into()),
    //             ("data_type".into(), data_type_value),
    //             ("field_type".into(), field_type_value),
    //         ]
    //             .into();
    // 
    //         collection_fields.push(content_field.into());
    //     }
    // 
    // 
    //     let data: BTreeMap<String, Value> = [
    //         ("name".into(), creatable_model.name.into()),
    //         ("identifier".into(), creatable_model.identifier.into()),
    //         (
    //             "created_by".into(),
    //             creatable_model.logged_in_username.clone().into(),
    //         ),
    //         (
    //             "updated_by".into(),
    //             creatable_model.logged_in_username.into(),
    //         ),
    //         ("collection_fields".into(), collection_fields.into()),
    //         ("created_at".into(), Datetime::default().into()),
    //         ("updated_at".into(), Datetime::default().into()),
    //     ]
    //         .into();
    // 
    //     let vars: BTreeMap<String, Value> = [
    //         ("data".into(), data.into()),
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
    // 
    //     let model: Result<CollectionModel> = result_object?.try_into();
    // 
    //     model
    // }
}
