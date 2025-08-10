use crate::error::{Error, Result};
use crate::models::{BaseModel, Pagination};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use surrealdb::sql::{Datetime, Object};

/// Represents a collection model in the system.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CollectionModel {

    /// Unique identifier for the collection.
    pub id: String,

    /// Name of the collection.
    pub name: String,

    /// Identifier for the collection, used for unique identification.
    pub identifier: String,

    /// Timestamps for creation and last update.
    pub created_at: Datetime,

    /// Timestamp for the last update of the collection.
    pub updated_at: Datetime,

    /// Username of the user who created the collection.
    pub created_by: String,

    /// Username of the user who last updated the collection.
    pub updated_by: String,
    // pub collection_fields: Vec<CollectionFieldModel>,
}

// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct CollectionFieldModel {
//     pub name: String,
//     pub identifier: String,
//     pub data_type: CollectionFieldDataType,
//     pub field_type: CollectionFieldFieldType,
// }


/// Represents a paginated response for collections.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CollectionPagination {

    /// A vector of collection models.
    pub data: Vec<CollectionModel>,

    /// Pagination information for the collection data.
    pub pagination: Pagination,
}


/// Represents a creatable collection model.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableCollection {

    /// Name of the collection to be created.
    pub name: String,

    /// Unique identifier for the collection.
    pub identifier: String,
    
    /// Username of the user creating the collection.
    pub logged_in_username: String,
    // pub collection_fields: Vec<CreatableCollectionField>,
}

// #[derive(Serialize, Debug, Deserialize, Clone)]
// pub struct CreatableCollectionField {
//     pub name: String,
//     pub identifier: String,
//     pub data_type: CollectionFieldDataType,
//     pub field_type: CollectionFieldFieldType,
// }

/// Represents an updatable collection model.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableCollection {

    /// Unique identifier for the collection to be updated.
    pub id: String,

    /// Name of the collection to be updated.
    pub name: String,

    /// Unique identifier for the collection.
    pub identifier: String,

    /// Username of the user updating the collection.
    pub logged_in_username: String,
    // pub collection_fields: Vec<UpdatableCollectionField>,
}

// #[derive(Serialize, Debug, Deserialize, Clone)]
// pub struct UpdatableCollectionField {
//     pub name: String,
//     pub identifier: String,
//     pub data_type: CollectionFieldDataType,
//     pub field_type: CollectionFieldFieldType,
// }

/// Represents a model for updating the collection identifier.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutCollectionIdentifierModel {
    /// Unique identifier for the collection to be updated.
    pub id: String,

    /// New identifier for the collection.
    pub identifier: String,

    /// Username of the user updating the collection identifier.
    pub logged_in_username: String,
}



/// Represents a model for updating the collection name.
#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CollectionFieldDataType {

    /// Represents a text data type for collection fields.
    Text(String),
}

/// Represents the field type for collection fields.
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub enum CollectionFieldFieldType {

    /// Represents a text field type for collection fields.
    #[default]
    Text,
}

impl Default for CollectionFieldDataType {
    fn default() -> CollectionFieldDataType {
        CollectionFieldDataType::Text("Text".to_string())
    }
}

impl TryFrom<CollectionModel> for crate::api::proto::content::CollectionModel {
    type Error = Error;

    fn try_from(val: CollectionModel) -> Result<crate::api::proto::content::CollectionModel> {
        let chrono_utc_created_at = val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at = val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let model = crate::api::proto::content::CollectionModel {
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

impl TryFrom<Object> for CollectionModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<CollectionModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        // let collection_fields = match val.get("collection_fields") {
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
        //                 let content_field: CollectionFieldModel = object.try_into()?;
        //
        //                 arr.push(content_field)
        //             }
        //             arr
        //         }
        //         _ => Vec::new(),
        //     },
        //     None => Vec::new(),
        // };

        Ok(CollectionModel {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
            // collection_fields,
        })
    }
}

// impl TryFrom<Object> for CollectionFieldModel {
//     type Error = Error;
//     fn try_from(val: Object) -> Result<CollectionFieldModel> {
//         let name = val.get("name").get_string()?;
//         let identifier = val.get("identifier").get_string()?;
//         let data_type_str = val.get("data_type").get_string()?;
//
//         let data_type = match data_type_str.as_str() {
//             "TEXT" => CollectionFieldDataType::Text("TEXT".to_string()),
//             _ => CollectionFieldDataType::default(),
//         };
//
//         let field_type_str = val.get("field_type").get_string()?;
//         let field_type = match field_type_str.as_str() {
//             "Text" => CollectionFieldFieldType::Text,
//
//             _ => CollectionFieldFieldType::default(),
//         };
//
//
//         Ok(CollectionFieldModel {
//             name,
//             identifier,
//             data_type,
//             field_type,
//         })
//     }
// }
