use crate::error::{Error, Result};
use crate::models::visitor_log_model::{CreatableVisitorLogModel, VistitorLogModel};
use crate::repositories::into_iter_objects;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Datetime;
use std::collections::BTreeMap;
use surrealdb::sql::Value;

/// visitor log repository
#[derive(Clone)]
pub struct VisitorLogRepository {}

impl Default for VisitorLogRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl VisitorLogRepository {
    /// new instance password repository
    #[must_use] pub const fn new() -> Self {
        Self {}
    }

    /// create visitor log record in table
    pub async fn create(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_visitor_log: CreatableVisitorLogModel
    ) -> Result<VistitorLogModel> {

        let sql = "CREATE visitor_log CONTENT $data";

        let data: BTreeMap<String, Value> = [
            (
                "content_type".into(),
                creatable_visitor_log.content_type.into(),
            ),
            (
                "content_id".into(),
                creatable_visitor_log.content_id.into(),
            ),
            (
                "ip_address".into(),
                creatable_visitor_log.ip_address.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(ress)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let visitor_log_model: Result<VistitorLogModel> = result_object?.try_into();

        visitor_log_model

    }
}