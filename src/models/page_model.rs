use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

use super::Pagination;



// This one should contain components and components fields with content
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ComponentContentModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub fields: Vec<ComponentFieldModel>,
}


#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct ComponentFieldModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_content: String,
    pub field_data: Vec<PageComponentFieldDataOption>
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
        let id = match val.get("id") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };
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
        let fields = match val.get("fields") {
            Some(val) => {
                
                match val.clone() {
                    Value::Array(v) => {
                        let mut arr = Vec::new();

                        for array in v.into_iter() {
                            let object = match array.clone() {
                                Value::Object(v) => v,
                                _ => surrealdb::sql::Object::default(),
                            };

                            let order_product_model: ComponentFieldModel = object.try_into()?;

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
            id,
            name,
            identifier,
            fields
        })
    }
}

impl TryFrom<Object> for ComponentFieldModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<ComponentFieldModel> {
        let id = match val.get("id") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };
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


        let field_type = match val.get("field_type") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };



        let field_content = match val.get("field_content") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let field_data = match val.get("field_data") {
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
        
        Ok(ComponentFieldModel {
            id,
            name,
            identifier,
            field_type,
            field_content,
            field_data
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
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub fields: Vec<CreatableComponentFieldContentModel>,
}



#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct CreatableComponentFieldContentModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_content: String,
    pub field_data: Vec<CreatablePageComponentFieldDataModel>
}


#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct CreatablePageComponentFieldDataModel {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatablePageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub component_contents: Vec<UpdatableComponentContentModel>,
    pub logged_in_username: String,
}

// This one should contain components and components fields with content
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdatableComponentContentModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub fields: Vec<UpdatableComponentFieldContentModel>,
}



#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct UpdatableComponentFieldContentModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_content: String,
    pub field_data: Vec<UpdatablePageComponentFieldDataModel>
}


#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct UpdatablePageComponentFieldDataModel {
    pub label: String,
    pub value: String,
}