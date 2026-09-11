use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;
use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AttributeModel {
    pub id: String,
    pub entity_id: String,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub updated_at: Datetime,
    pub updated_by: String,
    pub deleted_at: Option<Datetime>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorableAttribute {
    // pub entity_id: String,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub logged_in_user_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdableIdentifierAttribute {
    pub identifier: String,
    pub logged_in_user_email: String,
}

#[cfg(feature = "ssr")]
impl TryFrom<surrealdb::types::Object> for AttributeModel {
    type Error = crate::error::Error;

    fn try_from(obj: surrealdb::types::Object) -> Result<Self> {
        use crate::core::domain::extensions::object_extension::ObjectExtension;

        let id = obj.get_id("id")?;
        let entity_id = obj.get_id("entity_id")?;

        let name = obj.get_string("name")?;
        let identifier = obj.get_string("identifier")?;
        let data_type = obj.get_string("data_type")?;
        let field_type = obj.get_string("field_type")?;
        
        let created_at = obj.get_datetime("created_at")?;
        let created_by = obj.get_string("created_by")?;

        let updated_at = obj.get_datetime("updated_at")?;
        let updated_by = obj.get_string("updated_by")?;

        let deleted_at = obj.get_optional_datetime("deleted_at")?;
        let deleted_by = obj.get_optional_string("deleted_by")?;

        Ok(AttributeModel {
            id,
            entity_id,
            name,
            identifier,
            data_type,
            field_type,
            created_at,
            created_by,
            updated_at,
            updated_by,
            deleted_at,
            deleted_by,
        })
    }
}
