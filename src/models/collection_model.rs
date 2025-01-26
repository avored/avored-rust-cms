use crate::error::Error;
use crate::models::{BaseModel, Pagination};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CollectionModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
    pub collection_fields: Vec<CollectionFieldModel>,
}


#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CollectionFieldModel {
    pub name: String,
    pub identifier: String,
    pub data_type: CollectionFieldDataType,
    pub field_type: CollectionFieldFieldType,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CollectionPagination {
    pub data: Vec<CollectionModel>,
    pub pagination: Pagination,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableCollection {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub collection_fields: Vec<CreatableCollectionField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableCollectionField {
    pub name: String,
    pub identifier: String,
    pub data_type: CollectionFieldDataType,
    pub field_type: CollectionFieldFieldType,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableCollection {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub logged_in_username: String,
    pub collection_fields: Vec<UpdatableCollectionField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableCollectionField {
    pub name: String,
    pub identifier: String,
    pub data_type: CollectionFieldDataType,
    pub field_type: CollectionFieldFieldType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutCollectionIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CollectionFieldDataType {
    Text(String),
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub enum CollectionFieldFieldType {
    #[default]
    Text
}

impl Default for CollectionFieldDataType {
    fn default() -> CollectionFieldDataType {
        CollectionFieldDataType::Text("Text".to_string())
    }
}


impl TryFrom<Object> for CollectionModel {
    type Error = Error;
    fn try_from(val: Object) -> crate::error::Result<CollectionModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        let collection_fields = match val.get("collection_fields") {
            Some(val) => match val.clone() {
                Value::Array(v) => {
                    let mut arr = Vec::new();

                    for array in v.into_iter() {
                        let object = match array.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        let content_field: CollectionFieldModel = object.try_into()?;

                        arr.push(content_field)
                    }
                    arr
                }
                _ => Vec::new(),
            },
            None => Vec::new(),
        };

        Ok(CollectionModel {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
            collection_fields,
        })
    }
}

impl TryFrom<Object> for CollectionFieldModel {
    type Error = Error;
    fn try_from(val: Object) -> crate::error::Result<CollectionFieldModel> {
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let data_type_str = val.get("data_type").get_string()?;

        let data_type = match data_type_str.as_str() {
            "TEXT" => CollectionFieldDataType::Text("TEXT".to_string()),
            _ => CollectionFieldDataType::default(),
        };

        let field_type_str = val.get("field_type").get_string()?;
        let field_type = match field_type_str.as_str() {
            "Text" => CollectionFieldFieldType::Text,

            _ => CollectionFieldFieldType::default(),
        };


        Ok(CollectionFieldModel {
            name,
            identifier,
            data_type,
            field_type,
        })
    }
}

