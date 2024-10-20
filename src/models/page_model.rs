use std::collections::BTreeMap;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use super::{BaseModel, Pagination};

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageFieldContentType {
    TextContentType {
        text_value: TextContentType
    },
    IntegerContentType {
        integer_value: IntegerContentType
    },
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct TextContentType {
    pub text_value: String
}


#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct IntegerContentType {
    pub integer_value: i64
}

impl Default for PageFieldContentType {
    fn default() -> PageFieldContentType {
        PageFieldContentType::TextContentType { text_value: TextContentType::default() }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum PageFieldType {
    Text,
    Textarea,
    Select,
    TextEditor,
    Radio
}

impl Default for PageFieldType {
    fn default() -> PageFieldType {
        PageFieldType::Text
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageFieldData {
    SelectFieldData {
        select_field_options: Vec<PageSelectFieldData>
    },
    RadioFieldData {
        radio_field_options: Vec<PageRadioFieldData>
    },
    None
}


impl Default for PageFieldData {
    fn default() -> PageFieldData {
        PageFieldData::None
    }
}


#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PageSelectFieldData {
    pub label: String,
    pub value: String
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PageRadioFieldData {
    pub label: String,
    pub value: String
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageDataType {
    Text(String)
}

impl Default for PageDataType {
    fn default() -> PageDataType {
        PageDataType::Text("TEXT".to_string())
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
    pub field_data: PageFieldData
}

impl TryFrom<TextContentType> for Value {
    type Error = Error;
    fn try_from(val: TextContentType) -> Result<Value> {

        let val_val: BTreeMap<String, Value> = [
            ("text_value".into(), val.text_value.into()),
        ].into();

        Ok(val_val.into())
    }
}


impl TryFrom<IntegerContentType> for Value {
    type Error = Error;
    fn try_from(val: IntegerContentType) -> Result<Value> {

        let val_val: BTreeMap<String, Value> = [
            ("integer_value".into(), val.integer_value.into()),
        ].into();

        Ok(val_val.into())
    }
}

impl TryFrom<PageSelectFieldData> for Value {
    type Error = Error;
    fn try_from(val: PageSelectFieldData) -> Result<Value> {

        let val_val: BTreeMap<String, Value> = [
            ("label".into(), val.label.into()),
            ("value".into(), val.value.into()),
        ].into();

        Ok(val_val.into())
    }
}

impl TryFrom<PageRadioFieldData> for Value {
    type Error = Error;
    fn try_from(val: PageRadioFieldData) -> Result<Value> {

        let val_val: BTreeMap<String, Value> = [
            ("label".into(), val.label.into()),
            ("value".into(), val.value.into()),
        ].into();

        Ok(val_val.into())
    }
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
                PageDataType::Text("TEXT".to_string())
            },

            _ => PageDataType::default()
        };

        let field_type_str = val.get("field_type").get_string()?;
        let field_type = match field_type_str.as_str() {
            "Text" => {
                PageFieldType::Text
            },
            "Textarea" => {
                PageFieldType::Textarea
            },
            "Select" => {
                PageFieldType::Select
            },
            "TextEditor" => {
                PageFieldType::TextEditor
            },
            "Radio" => {
                PageFieldType::Radio
            },

            _ => PageFieldType::default()
        };

        let field_content = match data_type_str.as_str() {
            "TEXT" => {
                let options = match val.get("field_content") {

                    Some(val) => {
                        let object = match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        println!("before test {:?}", object);
                        let option: TextContentType = object.try_into()?;
                        println!("test {:?}", option);

                        option
                    },
                    None => {
                        TextContentType::default()
                    },
                };

                PageFieldContentType::TextContentType {
                    text_value: options
                }
            }
            "INT" => {
                let options = match val.get("field_content") {

                    Some(val) => {
                        let object = match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        let option: IntegerContentType = object.try_into()?;

                        option
                    },
                    None => {
                        IntegerContentType {
                            integer_value: 0
                        }
                    },
                };

                PageFieldContentType::IntegerContentType {
                    integer_value: options
                }
            },
            _ => PageFieldContentType::default()
        };

        let field_data = match field_type_str.as_str() {
            "Select" => {
                let options = match val.get("field_data") {
                    Some(val) => {
                        match val.clone() {
                            Value::Array(v) => {
                                let mut arr = Vec::new();

                                for array in v.into_iter() {
                                    let object = match array.clone() {
                                        Value::Object(v) => v,
                                        _ => Object::default(),
                                    };

                                    let option: PageSelectFieldData = object.try_into()?;

                                    arr.push(option)
                                }

                                PageFieldData::SelectFieldData {
                                    select_field_options: arr
                                }
                            }
                            _ => {

                                PageFieldData::None
                            },
                        }
                    }
                    None => {
                        PageFieldData::None
                    },
                };

                options
            },
            "Radio" => {
                let options = match val.get("field_data") {
                    Some(val) => {
                        match val.clone() {
                            Value::Array(v) => {
                                let mut arr = Vec::new();

                                for array in v.into_iter() {
                                    let object = match array.clone() {
                                        Value::Object(v) => v,
                                        _ => Object::default(),
                                    };

                                    let option: PageRadioFieldData = object.try_into()?;

                                    arr.push(option)
                                }

                                PageFieldData::RadioFieldData {
                                    radio_field_options: arr
                                }
                            }
                            _ => {

                                PageFieldData::None
                            },
                        }
                    }
                    None => {
                        PageFieldData::None
                    },
                };

                options
            },

            _ => PageFieldData::None
        };




        Ok(PageFieldModel {
            name,
            identifier,
            data_type,
            field_type,
            field_content,
            field_data
        })
    }
}

impl TryFrom<Object> for TextContentType {
    type Error = Error;
    fn try_from(val: Object) -> Result<TextContentType> {
        let value = val.get("text_value").get_string()?;
        Ok(TextContentType {
            text_value: value
        })
    }
}

impl TryFrom<Object> for IntegerContentType {
    type Error = Error;
    fn try_from(val: Object) -> Result<IntegerContentType> {
        let value = val.get("value").get_int()?;
        Ok(IntegerContentType {
            integer_value: value
        })
    }
}

impl TryFrom<Object> for PageSelectFieldData {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageSelectFieldData> {
        let label = val.get("label").get_string()?;
        let value = val.get("value").get_string()?;
        Ok(PageSelectFieldData {
            label,
            value
        })
    }
}

impl TryFrom<Object> for PageRadioFieldData {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageRadioFieldData> {
        let label = val.get("label").get_string()?;
        let value = val.get("value").get_string()?;
        Ok(PageRadioFieldData {
            label,
            value
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
    pub field_data: PageFieldData
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
    pub field_content: PageFieldContentType,
    pub field_data: PageFieldData
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutPageIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String
}

