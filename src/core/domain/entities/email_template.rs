use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;

use crate::{core::domain::extensions::object_extension::ObjectExtension, error::Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EmailTemplateModel {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub body_html: String,
    pub body_plain: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted_at: Option<Datetime>,
    pub deleted_by: Option<String>,
}

#[cfg(feature = "ssr")]
impl TryFrom<surrealdb::types::Object> for EmailTemplateModel {
    type Error = crate::error::Error;

    fn try_from(obj: surrealdb::types::Object) -> Result<Self> {
        let id = obj.get_id("id")?;
        let name = obj.get_string("name")?;
        let subject = obj.get_string("subject")?;
        let body_html = obj.get_string("body_html")?;
        let body_plain = obj.get_optional_string("body_plain")?;
        let created_at = obj.get_datetime("created_at")?;
        let updated_at = obj.get_datetime("updated_at")?;
        let deleted_at = obj.get_optional_datetime("deleted_at")?;
        let deleted_by = obj.get_optional_string("deleted_by")?;

        Ok(Self {
            id,
            name,
            subject,
            body_html,
            body_plain,
            created_at,
            updated_at,
            deleted_at,
            deleted_by,
        })
    }
}
