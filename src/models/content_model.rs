use std::collections::BTreeMap;
use std::time::SystemTime;
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use crate::error::{Error, Result};
use crate::models::{BaseModel, Pagination};



// region: Content model structs and enums

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ContentPagination {
    pub data: Vec<ContentModel>,
    pub pagination: Pagination,
}
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ContentModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    // pub content_fields: Vec<ContentFieldModel>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ContentFieldModel {
    pub name: String,
    pub identifier: String,
    pub data_type: ContentDataType,
    pub field_type: ContentFieldType,
    pub field_content: ContentFieldContentType,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ContentDataType {
    Text(String),
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub enum ContentFieldType {
    #[default]
    Text
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ContentFieldContentType {
    ContentTextType { text_value: ContentTextType },
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct ContentTextType {
    pub text_value: String,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableContentModel {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub content_type: String,
    // pub content_fields: Vec<CreatableContentField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableContentField {
    pub name: String,
    pub identifier: String,
    pub data_type: ContentDataType,
    pub field_type: ContentFieldType,
    pub field_content: ContentFieldContentType,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableContentModel {
    pub id: String,
    pub name: String,
    pub content_type: String,
    pub logged_in_username: String,
    pub updated_at: Datetime,
    pub updated_by: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableContentField {
    pub name: String,
    pub identifier: String,
    pub data_type: ContentDataType,
    pub field_type: ContentFieldType,
    pub field_content: ContentFieldContentType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutContentIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub content_type: String,
    pub logged_in_username: String,
}

// endregion: Content model structs and enums


// region: impl Default for content model enums

impl Default for ContentDataType {
    fn default() -> ContentDataType {
        ContentDataType::Text("TEXT".to_string())
    }
}

impl Default for ContentFieldContentType {
    fn default() -> ContentFieldContentType {
        ContentFieldContentType::ContentTextType {
            text_value: ContentTextType::default(),
        }
    }
}

// endregion: impl Default for content model enums



// region: impl surreal Value for content model structs

impl TryFrom<ContentTextType> for Value {
    type Error = Error;
    fn try_from(val: ContentTextType) -> Result<Value> {
        let val_val: BTreeMap<String, Value> =
            [("text_value".into(), val.text_value.into())].into();

        Ok(val_val.into())
    }
}



// endregion: impl surreal Value for content model structs



// region: impl surreal Object for content model structs


impl TryFrom<ContentModel> for crate::api::proto::content::ContentModel {
    type Error = Error;

    fn try_from(val: ContentModel) -> Result<crate::api::proto::content::ContentModel> {
        let chrono_utc_created_at= val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at= val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let model = crate::api::proto::content::ContentModel {
            id: val.id,
            name: val.name,
            identifier: val.identifier,
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
        };

        Ok(model)
    }
}


impl TryFrom<Object> for ContentModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ContentModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;


        // let content_fields = match val.get("content_fields") {
        //     Some(val) => match val.clone() {
        //         Value::Array(v) => {
        //             let mut arr = Vec::new();
        // 
        //             for array in v.into_iter() {
        //                 let object = match array.clone() {
        //                     Value::Object(v) => v,
        //                     _ => Object::default(),
        //                 };
        // 
        //                 let content_field: ContentFieldModel = object.try_into()?;
        // 
        //                 arr.push(content_field)
        //             }
        //             arr
        //         }
        //         _ => Vec::new(),
        //     },
        //     None => Vec::new(),
        // };

        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(ContentModel {
            id,
            name,
            identifier,
            // content_fields,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

impl TryFrom<Object> for ContentFieldModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ContentFieldModel> {
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let data_type_str = val.get("data_type").get_string()?;

        let data_type = match data_type_str.as_str() {
            "TEXT" => ContentDataType::Text("TEXT".to_string()),
            _ => ContentDataType::default(),
        };

        let field_type_str = val.get("field_type").get_string()?;
        let field_type = match field_type_str.as_str() {
            "Text" => ContentFieldType::Text,

            _ => ContentFieldType::default(),
        };

        let field_content = match data_type_str.as_str() {
            "TEXT" => {
                let options = match val.get("field_content") {
                    Some(val) => {
                        let object = match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        // println!("before test {:?}", object);
                        let option: ContentTextType = object.try_into()?;
                        // println!("test {:?}", option);

                        option
                    }
                    None => ContentTextType::default(),
                };

                ContentFieldContentType::ContentTextType {
                    text_value: options,
                }
            }

            _ => ContentFieldContentType::default(),
        };


        Ok(ContentFieldModel {
            name,
            identifier,
            data_type,
            field_type,
            field_content,
        })
    }
}

impl TryFrom<Object> for ContentTextType {
    type Error = Error;
    fn try_from(val: Object) -> Result<ContentTextType> {
        let value = val.get("text_value").get_string()?;
        Ok(ContentTextType { text_value: value })
    }
}


// endregion: impl surreal Object for content model structs


// region: content model creatable and updatable structs


// endregion: content model creatable and updatable structs