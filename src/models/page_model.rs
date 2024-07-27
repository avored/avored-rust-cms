use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

use super::Pagination;



// This one should contain components and components fields with content
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ComponentContentModel {
    pub name: String,
    pub identifier: String,
    pub elements: Vec<ComponentElementModel>,
}


#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct ComponentElementModel {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_content: String,
    pub element_data: Vec<PageComponentFieldDataOption>
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PageComponentFieldDataOption {
    pub label: String,
    pub value: String
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub components_content: Vec<ComponentContentModel>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl TryFrom<Object> for PageModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageModel> {
        let id = match val.get("id") {
            Some(val) => match val.clone() {
                Value::Thing(v) => {
                    let id = v.id;
                    id.to_string()
                }
                _ => String::from(""),
            },
            None => String::from(""),
        };
        let name = match val.get("name") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let identifier = match val.get("identifier") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let components_content = match val.get("components_content") {
            Some(val) => {
                
                match val.clone() {
                    Value::Array(v) => {
                        let mut arr = Vec::new();

                        for array in v.into_iter() {
                            let object = match array.clone() {
                                Value::Object(v) => v,
                                _ => surrealdb::sql::Object::default(),
                            };

                            let order_product_model: ComponentContentModel = object.try_into()?;

                            arr.push(order_product_model)
                        }
                        arr
                    }
                    _ => Vec::new(),
                }
            }
            None => Vec::new(),
        };

        let created_at = match val.get("created_at") {
            Some(val) => match val.clone() {
                Value::Datetime(v) => v,
                _ => Datetime::default(),
            },
            None => Datetime::default(),
        };
        let updated_at = match val.get("updated_at") {
            Some(val) => match val.clone() {
                Value::Datetime(v) => v,
                _ => Datetime::default(),
            },
            None => Datetime::default(),
        };

        let created_by = match val.get("created_by") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let updated_by = match val.get("updated_by") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        Ok(PageModel {
            id,
            name,
            identifier,
            components_content,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}


impl TryFrom<Object> for ComponentContentModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ComponentContentModel> {
        let name = match val.get("name") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let identifier = match val.get("identifier") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };
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

                            let order_product_model: ComponentElementModel = object.try_into()?;

                            arr.push(order_product_model)
                        }
                        arr
                    }
                    _ => Vec::new(),
                }
            }
            None => Vec::new(),
        };

        Ok(ComponentContentModel {
            name,
            identifier,
            elements
        })
    }
}

impl TryFrom<Object> for ComponentElementModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ComponentElementModel> {

        let name = match val.get("name") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let identifier = match val.get("identifier") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };


        let element_type = match val.get("element_type") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };



        let element_content = match val.get("element_content") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

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

                            let field_data_option: PageComponentFieldDataOption = object.try_into()?;

                            arr.push(field_data_option)
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
            element_content,
            element_data
        })
    }
}

impl TryFrom<Object> for PageComponentFieldDataOption {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageComponentFieldDataOption> {
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


        Ok(PageComponentFieldDataOption {
            label,
            value
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PagePagination {
    pub data: Vec<PageModel>,
    pub pagination: Pagination,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatablePageModel {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub component_contents: Vec<CreatableComponentContentModel>,
}


// This one should contain components and components fields with content
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatableComponentContentModel {
    pub name: String,
    pub identifier: String,
    pub elements: Vec<CreatableComponentElementContentModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutPageIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct CreatableComponentElementContentModel {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_content: String,
    pub element_data: Vec<CreatablePageComponentElementDataModel>
}


#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct CreatablePageComponentElementDataModel {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatablePageModel {
    pub id: String,
    pub name: String,
    pub component_contents: Vec<UpdatableComponentContentModel>,
    pub logged_in_username: String,
}

// This one should contain components and components fields with content
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdatableComponentContentModel {
    pub name: String,
    pub identifier: String,
    pub elements: Vec<UpdatableComponentElementContentModel>,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct UpdatableComponentElementContentModel {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_content: String,
    pub element_data: Vec<UpdatablePageComponentElementDataModel>
}


#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct UpdatablePageComponentElementDataModel {
    pub label: String,
    pub value: String,
}