use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::page_model::PageModel;
use crate::models::ModelCount;
use crate::PER_PAGE;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;

use super::into_iter_objects;

pub struct PageRepository {}

impl PageRepository {
    pub fn new() -> Self {
        PageRepository {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
    ) -> Result<Vec<PageModel>> {
        let sql = "SELECT * FROM pages LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut page_list: Vec<PageModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let page_object = object?;

            let page_model: Result<PageModel> = page_object.try_into();
            page_list.push(page_model?);
        }
        Ok(page_list)
    }

    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM pages GROUP ALL;";
        let responses = datastore.execute(sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }
}
