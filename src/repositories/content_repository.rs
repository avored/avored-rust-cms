use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};
use crate::error::Error;
use crate::models::content_model::{ContentDataType, ContentFieldContentType, ContentFieldType, ContentModel, CreatableContentModel};
use crate::repositories::into_iter_objects;
use crate::error::Result;

#[derive(Clone)]
pub struct ContentRepository {}

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
}

impl ContentRepository {
    pub fn new() -> Self {
        ContentRepository {}
    }
}