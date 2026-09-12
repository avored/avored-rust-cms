use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;
use crate::{core::domain::entities::AttributeModel, error::Result};

#[cfg(feature = "ssr")]
use crate::core::domain::extensions::object_extension::ObjectExtension;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EntityModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub updated_at: Datetime,
    pub updated_by: String,
    pub deleted_at: Option<Datetime>,
    pub deleted_by: Option<String>,
    pub attributes: Vec<AttributeModel>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorableEntityAttribute {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorableEntity {
    pub name: String,
    pub identifier: String,
    pub logged_in_user_email: String,
    pub attributes: Vec<StorableEntityAttribute>
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatableEntityAttribute {
    pub id: Option<String>,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub is_new: bool
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatableEntity {
    pub name: String,
    pub logged_in_user_email: String,
    pub attributes: Vec<UpdatableEntityAttribute>
}

#[cfg(feature = "ssr")]
impl TryFrom<surrealdb::types::Object> for EntityModel {
    type Error = crate::error::Error;

    fn try_from(obj: surrealdb::types::Object) -> Result<Self> {
        let id = obj.get_id("id")?;

        let name = obj.get_string("name")?;
        let identifier = obj.get_string("identifier")?;

        let created_at = obj.get_datetime("created_at")?;
        let created_by = obj.get_string("created_by")?;

        let updated_at = obj.get_datetime("updated_at")?;
        let updated_by = obj.get_string("updated_by")?;

        let deleted_at = obj.get_optional_datetime("deleted_at")?;
        let deleted_by = obj.get_optional_string("deleted_by")?;

        // Parse the inlined attributes subquery result
        let attributes = match obj.get("attributes") {
            Some(surrealdb::types::Value::Array(arr)) => {
                let mut attrs = Vec::new();
                for val in arr.iter() {
                    if let surrealdb::types::Value::Object(attr_obj) = val {
                        let attribute: AttributeModel = attr_obj.clone().try_into()?;
                        attrs.push(attribute);
                    }
                }
                attrs
            }
            _ => vec![],
        };

        Ok(EntityModel {
            id,
            name,
            identifier,
            created_at,
            created_by,
            updated_at,
            updated_by,
            deleted_at,
            deleted_by,
            attributes,
        })
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdableIdentifierEntity {
    pub identifier: String,
    pub logged_in_user_email: String,
}
