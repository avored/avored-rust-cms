use super::{BaseModel, Pagination};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{Datetime, Object, Value};

// region: Page model structs and enums

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub status: PageStatus,
    pub page_fields: Vec<PageFieldModel>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub enum PageStatus {
    #[default]
    Draft,
    Published,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PageFieldModel {
    pub name: String,
    pub identifier: String,
    pub data_type: PageDataType,
    pub field_type: PageFieldType,
    pub field_content: PageFieldContentType,
    pub field_data: PageFieldData,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageDataType {
    Text(String),
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub enum PageFieldType {
    #[default]
    Text,
    Textarea,
    Select,
    TextEditor,
    Radio,
    Checkbox,
    SingleImage,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageFieldContentType {
    TextContentType { text_value: TextContentType },
    IntegerContentType { integer_value: IntegerContentType },
    ArrayContentType { array_value: ArrayContentType },
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct TextContentType {
    pub text_value: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct IntegerContentType {
    pub integer_value: i64,
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct ArrayContentType {
    pub array_value: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageFieldData {
    SelectFieldData {
        select_field_options: Vec<PageSelectFieldData>,
    },
    RadioFieldData {
        radio_field_options: Vec<PageRadioFieldData>,
    },
    CheckboxFieldData {
        checkbox_field_options: Vec<PageCheckboxFieldData>,
    },
    NoneFieldData {
        none: String,
    },
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PageSelectFieldData {
    pub label: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PageRadioFieldData {
    pub label: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PageCheckboxFieldData {
    pub label: String,
    pub value: String,
}

// endregion: Page model structs and enums

// region: impl Default for page model enums

impl Default for PageFieldContentType {
    fn default() -> PageFieldContentType {
        PageFieldContentType::TextContentType {
            text_value: TextContentType::default(),
        }
    }
}

impl Default for PageFieldData {
    fn default() -> PageFieldData {
        PageFieldData::NoneFieldData {
            none: String::from(""),
        }
    }
}

//
// impl Default for PageStatus {
//     fn default() -> PageStatus {
//         PageStatus::Draft
//     }
// }

impl Default for PageDataType {
    fn default() -> PageDataType {
        PageDataType::Text("TEXT".to_string())
    }
}

// endregion: impl Default for page model enums

// region: impl surreal Value for page model structs

impl TryFrom<TextContentType> for Value {
    type Error = Error;
    fn try_from(val: TextContentType) -> Result<Value> {
        let val_val: BTreeMap<String, Value> =
            [("text_value".into(), val.text_value.into())].into();

        Ok(val_val.into())
    }
}

impl TryFrom<IntegerContentType> for Value {
    type Error = Error;
    fn try_from(val: IntegerContentType) -> Result<Value> {
        let val_val: BTreeMap<String, Value> =
            [("integer_value".into(), val.integer_value.into())].into();

        Ok(val_val.into())
    }
}

impl TryFrom<ArrayContentType> for Value {
    type Error = Error;
    fn try_from(val: ArrayContentType) -> Result<Value> {
        let val_val: BTreeMap<String, Value> =
            [("array_value".into(), val.array_value.into())].into();

        Ok(val_val.into())
    }
}

impl TryFrom<PageSelectFieldData> for Value {
    type Error = Error;
    fn try_from(val: PageSelectFieldData) -> Result<Value> {
        let val_val: BTreeMap<String, Value> = [
            ("label".into(), val.label.into()),
            ("value".into(), val.value.into()),
        ]
        .into();

        Ok(val_val.into())
    }
}

impl TryFrom<PageRadioFieldData> for Value {
    type Error = Error;
    fn try_from(val: PageRadioFieldData) -> Result<Value> {
        let val_val: BTreeMap<String, Value> = [
            ("label".into(), val.label.into()),
            ("value".into(), val.value.into()),
        ]
        .into();

        Ok(val_val.into())
    }
}

impl TryFrom<PageCheckboxFieldData> for Value {
    type Error = Error;
    fn try_from(val: PageCheckboxFieldData) -> Result<Value> {
        let val_val: BTreeMap<String, Value> = [
            ("label".into(), val.label.into()),
            ("value".into(), val.value.into()),
        ]
        .into();

        Ok(val_val.into())
    }
}

// endregion: impl surreal Value for page model structs

// region: impl surreal Object for page model structs
impl TryFrom<Object> for PageModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let status = match val.get("status").get_string()?.as_str() {
            "Draft" => PageStatus::Draft,
            "Published" => PageStatus::Published,
            _ => PageStatus::default(),
        };

        let page_fields = match val.get("page_fields") {
            Some(val) => match val.clone() {
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
            },
            None => Vec::new(),
        };

        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(PageModel {
            id,
            name,
            identifier,
            status,
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
            "TEXT" => PageDataType::Text("TEXT".to_string()),
            "Array_Text" => PageDataType::Text("Array_Text".to_string()),

            _ => PageDataType::default(),
        };

        let field_type_str = val.get("field_type").get_string()?;
        let field_type = match field_type_str.as_str() {
            "Text" => PageFieldType::Text,
            "Textarea" => PageFieldType::Textarea,
            "Select" => PageFieldType::Select,
            "TextEditor" => PageFieldType::TextEditor,
            "Radio" => PageFieldType::Radio,
            "Checkbox" => PageFieldType::Checkbox,
            "SingleImage" => PageFieldType::SingleImage,

            _ => PageFieldType::default(),
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
                        let option: TextContentType = object.try_into()?;
                        // println!("test {:?}", option);

                        option
                    }
                    None => TextContentType::default(),
                };

                PageFieldContentType::TextContentType {
                    text_value: options,
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
                    }
                    None => IntegerContentType { integer_value: 0 },
                };

                PageFieldContentType::IntegerContentType {
                    integer_value: options,
                }
            }
            "Array_Text" => {
                let array_val = match val.get("field_content") {
                    Some(val) => {
                        let object = match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };
                        println!("before obj 0 {:?}", object);
                        let option: ArrayContentType = object.try_into()?;

                        option
                    }
                    None => ArrayContentType {
                        array_value: vec![],
                    },
                };
                // let array_obj = ArrayContentType {
                //     array_value: array_val
                // };
                PageFieldContentType::ArrayContentType {
                    array_value: array_val,
                }
            }
            _ => PageFieldContentType::default(),
        };

        let field_data = match field_type_str.as_str() {
            "Select" => {
                let options = match val.get("field_data") {
                    Some(val) => match val.clone() {
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
                                select_field_options: arr,
                            }
                        }
                        _ => PageFieldData::NoneFieldData {
                            none: String::from(""),
                        },
                    },
                    None => PageFieldData::NoneFieldData {
                        none: String::from(""),
                    },
                };

                options
            }

            "Radio" => {
                let options = match val.get("field_data") {
                    Some(val) => match val.clone() {
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
                                radio_field_options: arr,
                            }
                        }
                        _ => PageFieldData::NoneFieldData {
                            none: String::from(""),
                        },
                    },
                    None => PageFieldData::NoneFieldData {
                        none: String::from(""),
                    },
                };

                options
            }

            "Checkbox" => {
                let options = match val.get("field_data") {
                    Some(val) => match val.clone() {
                        Value::Array(v) => {
                            let mut arr = Vec::new();

                            for array in v.into_iter() {
                                let object = match array.clone() {
                                    Value::Object(v) => v,
                                    _ => Object::default(),
                                };

                                let option: PageCheckboxFieldData = object.try_into()?;

                                arr.push(option)
                            }

                            PageFieldData::CheckboxFieldData {
                                checkbox_field_options: arr,
                            }
                        }
                        _ => PageFieldData::NoneFieldData {
                            none: String::from(""),
                        },
                    },
                    None => PageFieldData::NoneFieldData {
                        none: String::from(""),
                    },
                };

                options
            }

            _ => PageFieldData::NoneFieldData {
                none: String::from(""),
            },
        };

        Ok(PageFieldModel {
            name,
            identifier,
            data_type,
            field_type,
            field_content,
            field_data,
        })
    }
}

impl TryFrom<Object> for TextContentType {
    type Error = Error;
    fn try_from(val: Object) -> Result<TextContentType> {
        let value = val.get("text_value").get_string()?;
        Ok(TextContentType { text_value: value })
    }
}

impl TryFrom<Object> for IntegerContentType {
    type Error = Error;
    fn try_from(val: Object) -> Result<IntegerContentType> {
        let value = val.get("value").get_int()?;
        Ok(IntegerContentType {
            integer_value: value,
        })
    }
}

impl TryFrom<Object> for ArrayContentType {
    type Error = Error;
    fn try_from(val: Object) -> Result<ArrayContentType> {
        let array_value = match val.get("array_value") {
            Some(val) => match val.clone() {
                Value::Array(v) => {
                    let mut arr = Vec::new();
                    for array in v.into_iter() {
                        arr.push(array.as_string())
                    }
                    arr
                }
                _ => Vec::new(),
            },
            None => Vec::new(),
        };

        Ok(ArrayContentType { array_value })
    }
}

impl TryFrom<Object> for PageSelectFieldData {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageSelectFieldData> {
        let label = val.get("label").get_string()?;
        let value = val.get("value").get_string()?;
        Ok(PageSelectFieldData { label, value })
    }
}

impl TryFrom<Object> for PageCheckboxFieldData {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageCheckboxFieldData> {
        let label = val.get("label").get_string()?;
        let value = val.get("value").get_string()?;
        Ok(PageCheckboxFieldData { label, value })
    }
}

impl TryFrom<Object> for PageRadioFieldData {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageRadioFieldData> {
        let label = val.get("label").get_string()?;
        let value = val.get("value").get_string()?;
        Ok(PageRadioFieldData { label, value })
    }
}

// endregion: impl surreal Value for page model structs

// region: page model creatable and updatable structs

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PagePagination {
    pub data: Vec<PageModel>,
    pub pagination: Pagination,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatablePageModel {
    pub name: String,
    pub identifier: String,
    pub status: PageStatus,
    pub logged_in_username: String,
    pub page_fields: Vec<CreatablePageField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: PageDataType,
    pub field_type: PageFieldType,
    pub field_content: PageFieldContentType,
    pub field_data: PageFieldData,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatablePageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub status: PageStatus,
    pub logged_in_username: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub page_fields: Vec<UpdatablePageField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: PageDataType,
    pub field_type: PageFieldType,
    pub field_content: PageFieldContentType,
    pub field_data: PageFieldData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutPageIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String,
}

// endregion: page model creatable and updatable structs
