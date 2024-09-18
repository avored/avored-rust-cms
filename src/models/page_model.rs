use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use super::{BaseModel, Pagination};

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageFieldContentType {
    StringType(String),
    Int64(i64)
}

impl Default for PageFieldContentType {
    fn default() -> PageFieldContentType {
        PageFieldContentType::StringType("".to_string())
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageFieldType {
    TEXT(String),
    TEXTAREA(String)
}

impl Default for PageFieldType {
    fn default() -> PageFieldType {
        PageFieldType::TEXT("TEXT".to_string())
    }
}


#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageDataType {
    TEXT(String)
}

impl Default for PageDataType {
    fn default() -> PageDataType {
        PageDataType::TEXT("TEXT".to_string())
    }
}


#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct NewPageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub page_fields: Vec<PageFieldModel>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PageFieldModel {
    pub name: String,
    pub identifier: String,
    pub data_type: PageDataType,
    pub field_type: PageFieldType,
    pub field_content: PageFieldContentType,
}


impl TryFrom<Object> for NewPageModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<NewPageModel> {

        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;

        let page_fields = match val.get("page_fields") {
                        Some(val) => {

                            match val.clone() {
                                Value::Array(v) => {
                                    let mut arr = Vec::new();

                                    for array in v.into_iter() {
                                        let object = match array.clone() {
                                            Value::Object(v) => v,
                                            _ => Object::default(),
                                        };

                                        let page_field: PageFieldModel = object.try_into()?;

                                        arr.push(page_field)
                                    }
                                    arr
                                }
                                _ => Vec::new(),
                            }
                        }
                        None => Vec::new(),
                    };


        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(NewPageModel {
            id,
            name,
            identifier,
            page_fields,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

impl TryFrom<Object> for PageFieldModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageFieldModel> {
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let data_type_str = val.get("data_type").get_string()?;

        let data_type = match data_type_str.as_str() {
            "TEXT" => {
                PageDataType::TEXT("TEXT".to_string())
            },

            _ => PageDataType::default()
        };

        let field_type_str = val.get("field_type").get_string()?;
        let field_type = match field_type_str.as_str() {
            "TEXT" => {
                PageFieldType::TEXT("TEXT".to_string())
            },
            "TEXTAREA" => {
                PageFieldType::TEXTAREA("TEXTAREA".to_string())
            },

            _ => PageFieldType::default()
        };

        let field_content = match data_type_str.as_str() {
            "TEXT" => {
                let value = val.get("field_content").get_string()?;

                PageFieldContentType::StringType(value)
            },
            "INT" => {
                let value = val.get("field_content").get_int()?;
                PageFieldContentType::Int64(value)
            },
            _ => PageFieldContentType::default()
        };

        Ok(PageFieldModel {
            name,
            identifier,
            data_type,
            field_type,
            field_content,
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PagePagination {
    pub data: Vec<NewPageModel>,
    pub pagination: Pagination,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct NewCreatablePageModel {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub page_fields: Vec<CreatablePageField>
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: PageDataType,
    pub field_type: PageFieldType,
    pub field_content: PageFieldContentType,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct NewUpdatablePageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub page_fields: Vec<UpdatablePageField>
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: PageDataType,
    pub field_type: PageFieldType,
    pub field_content: PageFieldContentType
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutPageIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String
}

