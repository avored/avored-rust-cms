use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};
use crate::error::Error;
use crate::models::content_model::{ContentDataType, ContentFieldContentType, ContentFieldType, ContentModel, CreatableContentModel, PutContentIdentifierModel, UpdatableContentModel};
use crate::repositories::into_iter_objects;
use crate::error::Result;
use crate::models::ModelCount;
use crate::PER_PAGE;

#[derive(Clone)]
pub struct ContentRepository {}

impl ContentRepository {

    pub(crate) async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        content_type: String,
        id: &str,
    ) -> Result<ContentModel> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), id.into()),
            ("table".into(), content_type.into()),
        ]
            .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        let model: Result<ContentModel> = result_object?.try_into();

        model
    }

    pub(crate) async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        content_type: &str,
        start: i64,
        order_column: String,
        order_type: String,
    ) -> Result<Vec<ContentModel>> {
        let sql = format!(
            "\
            SELECT * \
            FROM type::table($table) \
            ORDER {} {}
            LIMIT $limit \
            START $start;\
        ",
            order_column, order_type
        );
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
            ("table".into(), content_type.into()),
        ]);
        let responses = datastore
            .execute(&sql, database_session, Some(vars))
            .await?;

        let mut content_list: Vec<ContentModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let content_object = object?;

            let content_model: Result<ContentModel> = content_object.try_into();
            content_list.push(content_model?);
        }
        Ok(content_list)
    }
    pub(crate) async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        content_type: &str
    ) -> Result<ModelCount> {
        let sql = format!("SELECT count() FROM {content_type} GROUP ALL;");
        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }

}

impl ContentRepository {
    pub(crate) async fn create_content(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_content_model: CreatableContentModel,
    ) -> Result<ContentModel>{
        let sql = "CREATE type::table($table) CONTENT $data";

        let mut content_fields: Vec<Value> = vec![];

        for created_content_field in creatable_content_model.content_fields {
            let data_type_value: Value = match created_content_field.data_type {
                ContentDataType::Text(v) => v.into(),
            };

            let field_type_value: Value = match created_content_field.field_type {
                ContentFieldType::Text => "Text".into(),
            };

            let field_content_value: Value = match created_content_field.field_content {
                ContentFieldContentType::ContentTextType { text_value } => text_value.try_into()?,
            };


            let content_field: BTreeMap<String, Value> = [
                ("name".into(), created_content_field.name.into()),
                ("identifier".into(), created_content_field.identifier.into()),
                ("data_type".into(), data_type_value),
                ("field_type".into(), field_type_value),
                ("field_content".into(), field_content_value),
            ]
                .into();

            content_fields.push(content_field.into());
        }


        let data: BTreeMap<String, Value> = [
            ("name".into(), creatable_content_model.name.into()),
            ("identifier".into(), creatable_content_model.identifier.into()),
            (
                "created_by".into(),
                creatable_content_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                creatable_content_model.logged_in_username.into(),
            ),
            ("content_fields".into(), content_fields.into()),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
            .into();

        let vars: BTreeMap<String, Value> = [
            ("data".into(), data.into()),
            ("table".into(), creatable_content_model.content_type.into()),
        ]
            .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        let model: Result<ContentModel> = result_object?.try_into();

        model
    }

    pub(crate) async fn update_content(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_model: UpdatableContentModel,
    ) -> Result<ContentModel> {
        let sql = "UPDATE type::thing($table, $id) CONTENT $data";

        let mut content_fields: Vec<Value> = vec![];

        for updatable_content_field in updatable_model.content_fields {
            let data_type_value: Value = match updatable_content_field.data_type {
                ContentDataType::Text(v) => v.into(),
            };

            let field_type_value: Value = match updatable_content_field.field_type {
                ContentFieldType::Text => "Text".into(),

            };
            let field_content_value: Value = match updatable_content_field.field_content {
                ContentFieldContentType::ContentTextType { text_value } => text_value.try_into()?,
            };

            let content_field: BTreeMap<String, Value> = [
                ("name".into(), updatable_content_field.name.into()),
                ("identifier".into(), updatable_content_field.identifier.into()),
                ("data_type".into(), data_type_value),
                ("field_type".into(), field_type_value),
                ("field_content".into(), field_content_value),
            ]
                .into();

            content_fields.push(content_field.into());
        }


        let data: BTreeMap<String, Value> = [
            ("name".into(), updatable_model.name.into()),
            ("identifier".into(), updatable_model.identifier.into()),
            (
                "updated_by".into(),
                updatable_model.logged_in_username.clone().into(),
            ),
            ("created_by".into(), updatable_model.created_by.into()),
            ("content_fields".into(), content_fields.into()),
            ("updated_at".into(), Datetime::default().into()),
            ("created_at".into(), updatable_model.created_at.into()),
        ]
            .into();

        let vars: BTreeMap<String, Value> = [
            ("data".into(), data.into()),
            ("table".into(), updatable_model.content_type.into()),
            ("id".into(), updatable_model.id.into()),
        ]
            .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        let model: Result<ContentModel> = result_object?.try_into();

        model
    }

    pub(crate) async fn count_of_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        collection_type: &str,
        identifier: &str
    ) -> Result<ModelCount> {
        let sql = format!("SELECT count(identifier=$identifier) FROM {collection_type} GROUP ALL");

        let vars: BTreeMap<String, Value> = [("identifier".into(), identifier.into())].into();
        let responses = datastore.execute(&sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }

    pub(crate) async fn update_content_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        put_content_identifier_model: PutContentIdentifierModel,
    ) -> Result<ContentModel> {
        let sql = "UPDATE type::thing($table, $id)
                    SET
                        identifier = $identifier,
                        updated_at = $updated_at,
                        updated_by = $updated_by
                    ;
        ";

        let vars: BTreeMap<String, Value> = [
            (
                "identifier".into(),
                put_content_identifier_model.identifier.into(),
            ),
            ("table".into(), put_content_identifier_model.collection_type.into()),
            ("updated_at".into(), Datetime::default().into()),
            (
                "updated_by".into(),
                put_content_identifier_model.logged_in_username.into(),
            ),
            ("id".into(), put_content_identifier_model.id.into()),
        ]
            .into();
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let updated_model: Result<ContentModel> = result_object?.try_into();

        updated_model
    }

    pub fn new() -> Self {
        ContentRepository {}
    }
}
