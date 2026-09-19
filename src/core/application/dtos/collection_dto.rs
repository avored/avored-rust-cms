use serde::{Deserialize, Serialize};

use crate::core::domain::entities::EntityModel;

/*  #region Paginate Command */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginateCollectionCommand {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

/* #endregion */

/* #region Collection Paginate response */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectionPaginationResponse {
    pub entity: EntityModel,
    pub data: Vec<serde_json::Value>,
    pub total: u64,
}
/* #endregion */

/* #region Create Collection Command */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCollectionCommand {
    #[serde(flatten)]
    pub fields: serde_json::Map<String, serde_json::Value>,
}
/* #endregion */

/* #region Collection Response */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectionResponse {
    pub record: serde_json::Value,
}
/* #endregion */

