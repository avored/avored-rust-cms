use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ContentModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub content_fields: Vec<ContentFieldModel>,
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
    TextContentType { text_value: ContentTextType },
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
    pub content_fields: Vec<CreatableContentField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableContentField {
    pub name: String,
    pub identifier: String,
    pub data_type: ContentDataType,
    pub field_type: ContentFieldType,
    pub field_content: ContentFieldContentType,
}


impl Default for ContentDataType {
    fn default() -> ContentDataType {
        ContentDataType::Text("TEXT".to_string())
    }
}

impl Default for ContentFieldContentType {
    fn default() -> ContentFieldContentType {
        ContentFieldContentType::TextContentType {
            text_value: ContentTextType::default(),
        }
    }
}