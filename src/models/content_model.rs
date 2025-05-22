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
pub struct ContentSelectFieldData {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ContentCheckboxFieldData {
    pub label: String,
    pub value: String,
}


#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ContentFieldData {
    pub content_select_field_options: Vec<ContentSelectFieldData>,
    pub content_checkbox_field_data: Vec<ContentCheckboxFieldData>,
}



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
    pub data_type: ContentFieldDataType,
    pub field_type: ContentFieldFieldType,
    pub field_content: ContentFieldFieldContent,
    pub field_data: Option<ContentFieldData>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ContentFieldDataType {
    Text,
    Int,
    Array,
    Float
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub enum ContentFieldFieldType {
    #[default]
    Text,
    Textarea,
    RichTextEditor,
    NumberTextField,
    FloatTextField,
    Select,
    Checkbox
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct ContentFieldFieldContent {
    pub text_value: Option<String>,
    pub int_value: Option<i64>,
    pub float_value: Option<f64>,
    pub array_value: Vec<String>,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableContentModel {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub content_type: String,
    pub content_fields: Vec<CreatableContentField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableContentField {
    pub name: String,
    pub identifier: String,
    pub data_type: ContentFieldDataType,
    pub field_type: ContentFieldFieldType,
    pub field_content: ContentFieldFieldContent,
    pub field_data: Option<ContentFieldData>,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableContentModel {
    pub id: String,
    pub name: String,
    pub content_type: String,
    pub logged_in_username: String,
    pub updated_at: Datetime,
    pub updated_by: String,
    pub content_fields: Vec<UpdatableContentField>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableContentField {
    pub name: String,
    pub identifier: String,
    pub data_type: ContentFieldDataType,
    pub field_type: ContentFieldFieldType,
    pub field_content: ContentFieldFieldContent,
    pub field_data: Option<ContentFieldData>,
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

impl Default for ContentFieldDataType {
    fn default() -> ContentFieldDataType {
        ContentFieldDataType::Text
    }
}



// endregion: impl Default for content model enums



// region:
// impl surreal Value for content model structs



// endregion: impl surreal Value for content model structs



// region: impl surreal Object for content model structs

impl TryFrom<ContentModel> for crate::api::proto::content::ContentModel {
    type Error = Error;

    fn try_from(val: ContentModel) -> Result<crate::api::proto::content::ContentModel> {
        let utc_created_at = val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let utc_updated_at = val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);
        
        let mut content_fields: Vec<crate::api::proto::content::ContentFieldModel> = vec![];
        
        for content_field in val.content_fields {
            let content_field_model: crate::api::proto::content::ContentFieldModel = content_field.try_into().unwrap();
            content_fields.push(content_field_model);
        }

        let model = crate::api::proto::content::ContentModel {
            id: val.id,
            name: val.name,
            identifier: val.identifier,
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
            content_fields,
            
        };

        Ok(model)
    }
}


impl TryFrom<ContentFieldFieldContent> for crate::api::proto::content::ContentFieldFieldContent {
    type Error = Error;

    fn try_from(val: ContentFieldFieldContent) -> Result<crate::api::proto::content::ContentFieldFieldContent > {

        // @todo think of a better way to do this
        // If string is empty then we should return None
        let model = crate::api::proto::content::ContentFieldFieldContent {
            text_value: Some(val.text_value.unwrap_or_default()),
            int_value: Some(val.int_value.unwrap_or_default()),
            float_value: Some(val.float_value.unwrap_or_default()),
            array_value: val.array_value
        };

        Ok(model)
    }
}


impl TryFrom<ContentFieldData> for crate::api::proto::content::ContentFieldData {
    type Error = Error;

    fn try_from(val: ContentFieldData) -> Result<crate::api::proto::content::ContentFieldData > {
        let mut options = vec![];
        
        for option in val.content_select_field_options {
            let t: crate::api::proto::content::ContentSelectFieldData = option.try_into()?;
            options.push(t);
        }

        let mut checkbox_options = vec![];
        for checkbox_option in val.content_checkbox_field_data {
            let t: crate::api::proto::content::ContentCheckboxFieldData = checkbox_option.try_into()?;
            checkbox_options.push(t);
        }


        // If string is empty then we should return None
        let model = crate::api::proto::content::ContentFieldData {
            content_select_field_options: options,
            content_checkbox_field_data: checkbox_options
        };

        Ok(model)
    }
}


impl TryFrom<ContentSelectFieldData> for crate::api::proto::content::ContentSelectFieldData {
    type Error = Error;

    fn try_from(val: ContentSelectFieldData) -> Result<crate::api::proto::content::ContentSelectFieldData > {

        // @todo think of a better way to do this
        // If string is empty then we should return None
        let model = crate::api::proto::content::ContentSelectFieldData {
            label: val.label,
            value: val.value,
        };

        Ok(model)
    }
}

impl TryFrom<ContentCheckboxFieldData> for crate::api::proto::content::ContentCheckboxFieldData {
    type Error = Error;

    fn try_from(val: ContentCheckboxFieldData) -> Result<crate::api::proto::content::ContentCheckboxFieldData > {

        let model = crate::api::proto::content::ContentCheckboxFieldData {
            label: val.label,
            value: val.value,
        };

        Ok(model)
    }
}


impl TryFrom<crate::api::proto::content::ContentCheckboxFieldData> for ContentCheckboxFieldData {
    type Error = Error;

    fn try_from(val: crate::api::proto::content::ContentCheckboxFieldData) -> Result<ContentCheckboxFieldData> {
        let content_select_field_data = ContentCheckboxFieldData {
            label: val.clone().label,
            value: val.value
        };

        Ok(content_select_field_data)
    }
}

impl TryFrom<crate::api::proto::content::ContentSelectFieldData> for ContentSelectFieldData {
    type Error = Error;

    fn try_from(val: crate::api::proto::content::ContentSelectFieldData) -> Result<ContentSelectFieldData> {
        let content_select_field_data = ContentSelectFieldData {
            label: val.clone().label,
            value: val.value
        };

        Ok(content_select_field_data)
    }
}

impl TryFrom<Option<crate::api::proto::content::ContentFieldData>> for ContentFieldData {
    type Error = Error;

    fn try_from(val: Option<crate::api::proto::content::ContentFieldData>) -> Result<ContentFieldData> {

        let content_field_data = match val {
            Some(val) => {
                let mut options: Vec<ContentSelectFieldData> = vec![];

                for option in val.content_select_field_options {
                    let t: ContentSelectFieldData = option.try_into()?;
                    options.push(t);
                }

                let mut checkbox_options: Vec<ContentCheckboxFieldData> = vec![];

                for checkbox_option in val.content_checkbox_field_data {
                    let t: ContentCheckboxFieldData = checkbox_option.try_into()?;
                    checkbox_options.push(t);
                }

               ContentFieldData {
                   content_select_field_options: options,
                   content_checkbox_field_data: checkbox_options,
               }
            },
            None => ContentFieldData::default()
        };

        Ok(content_field_data)
    }
}




impl TryFrom<ContentSelectFieldData> for Value {
    type Error = Error;
    fn try_from(val: ContentSelectFieldData) -> Result<Value> {
        let val_val: BTreeMap<String, Value> =
            [
                ("label".into(), val.label.into()),
                ("value".into(), val.value.into()),
            ].into();

        Ok(val_val.into())
    }
}

impl TryFrom<ContentCheckboxFieldData> for Value {
    type Error = Error;
    fn try_from(val: ContentCheckboxFieldData) -> Result<Value> {
        let val_val: BTreeMap<String, Value> =
            [
                ("label".into(), val.label.into()),
                ("value".into(), val.value.into()),
            ].into();

        Ok(val_val.into())
    }
}



impl TryFrom<String> for ContentFieldDataType {
    type Error = Error; 
    
    fn try_from(val: String) -> Result<ContentFieldDataType> {
        let data_type = match val.as_str() {
            "TEXT" => ContentFieldDataType::Text,
            "INT" => ContentFieldDataType::Int,
            "ARRAY" => ContentFieldDataType::Array,
            "FLOAT" => ContentFieldDataType::Float,
            _ => ContentFieldDataType::default(),
        };
        
        Ok(data_type)
    }
}


impl TryFrom<String> for ContentFieldFieldType {
    type Error = Error;

    fn try_from(val: String) -> Result<ContentFieldFieldType> {
        let field_type = match val.as_str() {
            "TEXT" => ContentFieldFieldType::Text,
            "TEXTAREA" => ContentFieldFieldType::Textarea,
            "RICH_TEXT_EDITOR" => ContentFieldFieldType::RichTextEditor,
            "NUMBER_TEXT_FIELD" => ContentFieldFieldType::NumberTextField,
            "FLOAT_TEXT_FIELD" => ContentFieldFieldType::FloatTextField,
            "Select" => ContentFieldFieldType::Select,
            "Checkbox" => ContentFieldFieldType::Checkbox,
            _ => ContentFieldFieldType::default(),
        };

        Ok(field_type)
    }
}


impl TryFrom<Option<crate::api::proto::content::ContentFieldFieldContent>> for ContentFieldFieldContent {
    type Error = Error;

    fn try_from(val: Option<crate::api::proto::content::ContentFieldFieldContent>) -> Result<ContentFieldFieldContent> {

        
        let content_field_field_content = ContentFieldFieldContent {
            text_value: val.clone().unwrap_or_default().text_value,
            int_value: val.clone().unwrap_or_default().int_value,
            float_value: val.clone().unwrap_or_default().float_value,
            array_value: val.unwrap_or_default().array_value,
        };

        Ok(content_field_field_content)
    }
}


impl TryFrom<ContentFieldFieldContent> for Value {
    type Error = Error;
    fn try_from(val: ContentFieldFieldContent) -> Result<Value> {
        let float_value = match val.float_value {
            Some(val) => val.into(),
            None => Value::None,
        };
        
        let val_val: BTreeMap<String, Value> =
            [
                ("text_value".into(), val.text_value.into()),
                ("int_value".into(), val.int_value.into()),
                ("array_value".into(), val.array_value.into()),
                ("float_value".into(), float_value),
            ].into();

        Ok(val_val.into())
    }
}

impl TryFrom<Object> for ContentFieldFieldContent {
    type Error = Error;
    fn try_from(val: Object) -> Result<ContentFieldFieldContent> {
        let value = val.get("text_value").get_string()?;
        let int_value = val.get("int_value").get_int()?;
        let float_value = val.get("float_value").get_float()?;
        
        let array_value: Vec<String> = match val.get("array_value") {
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


        Ok(ContentFieldFieldContent { 
            text_value: Some(value),
            int_value: Some(int_value),
            float_value: Some(float_value),
            array_value,
        })
    }
}


impl TryFrom<Option<ContentFieldFieldContent>> for crate::api::proto::content::ContentFieldFieldContent {
    type Error = Error;

    fn try_from(val: Option<ContentFieldFieldContent>) -> Result<crate::api::proto::content::ContentFieldFieldContent > {

        let field_content_content_type = match val {
            Some(val) => {
                let model = crate::api::proto::content::ContentFieldFieldContent {
                    text_value: Some(val.text_value.unwrap()),
                    int_value: Some(val.int_value.unwrap()),
                    array_value: val.array_value,
                    float_value: val.float_value,
                };
                
                model
            },
            None => {
                let model = crate::api::proto::content::ContentFieldFieldContent {
                    text_value: None,
                    int_value: None,
                    float_value: None,
                    array_value: vec![],
                };
                
                model
            }
        };
        
        Ok(field_content_content_type)
    }
}


impl TryFrom<ContentFieldModel> for crate::api::proto::content::ContentFieldModel {
    type Error = Error;

    fn try_from(val: ContentFieldModel) -> Result<crate::api::proto::content::ContentFieldModel > {
        
        let field_content: crate::api::proto::content::ContentFieldFieldContent  = val.field_content.try_into()?;
        let field_data: crate::api::proto::content::ContentFieldData  = match val.field_data {
            Some(val ) => val.try_into()?,
            None => crate::api::proto::content::ContentFieldData::default(),
        };
        
        let model = crate::api::proto::content::ContentFieldModel {
            name: val.name,
            identifier: val.identifier,
            data_type: val.data_type.try_into()?,
            field_type: val.field_type.try_into()?,
            field_content: Some(field_content),
            field_data: Some(field_data),
        };

        Ok(model)
    }
}

impl TryFrom<ContentFieldDataType> for String {
    type Error = Error;

    fn try_from(val: ContentFieldDataType) -> Result<String> {

        let string_val = match val {
            ContentFieldDataType::Text => String::from("TEXT"),
            ContentFieldDataType::Int => String::from("INT"),
            ContentFieldDataType::Array => String::from("ARRAY"),
            ContentFieldDataType::Float => String::from("FLOAT"),
        };

        Ok(string_val)
    }
}



impl TryFrom<ContentFieldFieldType> for String {
    type Error = Error;

    fn try_from(val: ContentFieldFieldType) -> Result<String> {

        let string_val = match val {
            ContentFieldFieldType::Text => String::from("TEXT"),
            ContentFieldFieldType::Textarea => String::from("TEXTAREA"),
            ContentFieldFieldType::RichTextEditor => String::from("RICH_TEXT_EDITOR"),
            ContentFieldFieldType::NumberTextField => String::from("NUMBER_TEXT_FIELD"),
            ContentFieldFieldType::FloatTextField => String::from("FLOAT_TEXT_FIELD"),
            ContentFieldFieldType::Select => String::from("Select"),
            ContentFieldFieldType::Checkbox => String::from("Checkbox"),
            
        };

        Ok(string_val)
    }
}




impl TryFrom<Object> for ContentModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ContentModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;


        let content_fields = match val.get("content_fields") {
            Some(val) => match val.clone() {
                Value::Array(v) => {
                    let mut arr = Vec::new();
        
                    for array in v.into_iter() {
                        let object = match array.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };
        
                        let content_field: ContentFieldModel = object.try_into()?;
        
                        arr.push(content_field)
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

        Ok(ContentModel {
            id,
            name,
            identifier,
            content_fields,
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
            "TEXT" => ContentFieldDataType::Text,
            "INT" => ContentFieldDataType::Int,
            "FLOAT" => ContentFieldDataType::Float,
            "ARRAY" => ContentFieldDataType::Array,
            _ => ContentFieldDataType::default(),
        };

        let field_type_str = val.get("field_type").get_string()?;
        let field_type = match field_type_str.as_str() {
            "Text" => ContentFieldFieldType::Text,
            "Textarea" => ContentFieldFieldType::Textarea,
            "RICH_TEXT_EDITOR" => ContentFieldFieldType::RichTextEditor,
            "NUMBER_TEXT_FIELD" => ContentFieldFieldType::NumberTextField,
            "FLOAT_TEXT_FIELD" => ContentFieldFieldType::FloatTextField,
            "Select" => ContentFieldFieldType::Select,
            "Checkbox" => ContentFieldFieldType::Checkbox,
            _ => ContentFieldFieldType::default(),
        };

        let field_content = match data_type_str.as_str() {
            "TEXT" => {
                let text_content_field_content = match val.get("field_content") {
                    Some(val) => {
                        let object = match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        let option: ContentFieldFieldContent = object.try_into()?;

                        option
                    }
                    None => ContentFieldFieldContent::default(),
                };

                text_content_field_content
            },
            "INT" => {
                let int_content_field_content = match val.get("field_content") {
                    Some(val) => {
                        let object = match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        let option: ContentFieldFieldContent = object.try_into()?;

                        option
                    }
                    None => ContentFieldFieldContent::default(),
                };

                int_content_field_content
            },
            "FLOAT" => {
                let float_content_field_content = match val.get("field_content") {
                    Some(val) => {
                        let object = match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        };

                        let option: ContentFieldFieldContent = object.try_into()?;

                        option
                    }
                    None => ContentFieldFieldContent::default(),
                };

                float_content_field_content
            }

            _ => ContentFieldFieldContent::default(),
        };


        let field_data = match field_type_str.as_str() {
            "Select" => {
                let content_data = match val.get("field_data") {
                    Some(val) => {

                        let object = match val.clone() {
                            Value::Array(v) => {
                                let mut arr: Vec<ContentSelectFieldData> = Vec::new();

                                for array in v.into_iter() {
                                    let object = match array.clone() {
                                        Value::Object(v) => v,
                                        _ => Object::default(),
                                    };

                                    let option: ContentSelectFieldData = object.try_into()?;
                                    arr.push(option)
                                }
                                
                                arr
                            },
                            _ => vec![]
                        };

                        println!("object {:?}", object);
                        // option
                        ContentFieldData {
                            content_select_field_options: object,
                            content_checkbox_field_data: vec![]
                        }
                    }
                    None => ContentFieldData::default(),
                };

                content_data
            },
            
            "Checkbox" => {
                let content_data = match val.get("field_data") {
                    Some(val) => {

                        let object = match val.clone() {
                            Value::Array(v) => {
                                let mut arr: Vec<ContentCheckboxFieldData> = Vec::new();

                                for array in v.into_iter() {
                                    let object = match array.clone() {
                                        Value::Object(v) => v,
                                        _ => Object::default(),
                                    };

                                    let option: ContentCheckboxFieldData = object.try_into()?;
                                    arr.push(option)
                                }

                                arr
                            },
                            _ => vec![]
                        };

                        println!("object {:?}", object);
                        // option
                        ContentFieldData {
                            content_select_field_options: vec![],
                            content_checkbox_field_data: object
                        }
                    }
                    None => ContentFieldData::default(),
                }; 
                
                content_data
            }
            

            _ => ContentFieldData::default(),
        };

        Ok(ContentFieldModel {
            name,
            identifier,
            data_type,
            field_type,
            field_content,
            field_data: Some(field_data),
        })
    }
}

// impl TryFrom<Object> for ContentFieldData {
//     type Error = Error;
//     fn try_from(val: Object) -> Result<ContentFieldData> {
//         
//         
//         let content_select_field_options = vec![];
// 
// 
//         Ok(ContentFieldData {
//             content_select_field_options
//         })
//     }
// }



impl TryFrom<Object> for ContentSelectFieldData {
    type Error = Error;
    fn try_from(val: Object) -> Result<ContentSelectFieldData> {
        let label = val.get("label").get_string()?;
        let value = val.get("value").get_string()?;


        Ok(ContentSelectFieldData {
            label,
            value,
        })
    }
}


impl TryFrom<Object> for ContentCheckboxFieldData {
    type Error = Error;
    fn try_from(val: Object) -> Result<ContentCheckboxFieldData> {
        let label = val.get("label").get_string()?;
        let value = val.get("value").get_string()?;


        Ok(ContentCheckboxFieldData {
            label,
            value,
        })
    }
}


// endregion: impl surreal Object for content model structs


// region: content model creatable and updatable structs


// endregion: content model creatable and updatable structs
