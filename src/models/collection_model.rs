use crate::error::Error;
use crate::models::{BaseModel, Pagination};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CollectionModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CollectionPagination {
    pub data: Vec<CollectionModel>,
    pub pagination: Pagination,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableCollection {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableCollection {
    pub id: String,
    pub name: String,
    pub logged_in_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutCollectionIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String,
}


impl TryFrom<Object> for CollectionModel {
    type Error = Error;
    fn try_from(val: Object) -> crate::error::Result<CollectionModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(CollectionModel {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}
