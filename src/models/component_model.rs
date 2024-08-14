use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use super::{BaseModel, Pagination};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ComponentModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
    pub elements: Vec<ComponentElementModel>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ComponentElementModel {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_data_type: String,
    pub element_data: Option<Vec<ComponentElementDataModel>>
}

impl TryFrom<Object> for ComponentModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ComponentModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        let elements = match val.get("elements") {
            Some(val) => {

                match val.clone() {
                    Value::Array(v) => {
                        let mut arr = Vec::new();

                        for array in v.into_iter() {
                            let object = match array.clone() {
                                Value::Object(v) => v,
                                _ => surrealdb::sql::Object::default(),
                            };

                            let component_element_model: ComponentElementModel = object.try_into()?;

                            arr.push(component_element_model)
                        }
                        arr
                    }
                    _ => Vec::new(),
                }
            }
            None => Vec::new(),
        };

        Ok(ComponentModel {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
            elements,
        })
    }
}


impl TryFrom<Object> for ComponentElementModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ComponentElementModel> {

        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let element_type = val.get("element_type").get_string()?;
        let element_data_type = val.get("element_data_type").get_string()?;
        let element_data = match val.get("element_data") {
            Some(val) => {

                match val.clone() {
                    Value::Array(v) => {
                        let mut arr = Vec::new();

                        for array in v.into_iter() {
                            let object = match array.clone() {
                                Value::Object(v) => v,
                                _ => surrealdb::sql::Object::default(),
                            };

                            let field_data_model: ComponentElementDataModel = object.try_into()?;

                            arr.push(field_data_model)
                        }
                        arr
                    }
                    _ => Vec::new(),
                }
            }
            None => Vec::new(),
        };

        Ok(ComponentElementModel {
            name,
            identifier,
            element_type,
            element_data_type,
            element_data :Some(element_data)
        })
    }
}



impl TryFrom<Object> for ComponentElementDataModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ComponentElementDataModel> {
        let label = match val.get("label") {
            Some(val) => {

                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };
        let value = match val.get("value") {
            Some(val) => {

                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };


        Ok(ComponentElementDataModel {
            label,
            value
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableComponent {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub elements: Vec<CreatableComponentElementModel>
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableComponentElementModel {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_data_type: String,
    pub element_data: Option<Vec<ComponentElementDataModel>>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutComponentIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableComponentModel {
    pub id: String,
    pub name: String,
    pub logged_in_username: String,
    pub elements: Vec<UpdatableComponentElementModel>
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableComponentElementModel {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_data_type: String,
    pub element_data: Option<Vec<ComponentElementDataModel>>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ComponentElementDataModel {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ComponentPagination {
    pub data: Vec<ComponentModel>,
    pub pagination: Pagination,
}
