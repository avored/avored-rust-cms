use crate::error::{Error, Result};
use crate::models::visitor_log_model::{CreatableVisitorLogModel, VisitByContentType, VisitByYear, VistitorLogModel};
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

        let sql = "CREATE visitor_logs CONTENT $data";

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

    /// Get visit by year
    pub async fn get_visit_by_year(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        year: i64
    ) -> Result<Vec<VisitByYear>> {

        println!("get_visit_by_year for year: {}", year);


        let sql: &'static str = "
            SELECT  count() AS visits,  
                time::format(created_at, '%b') AS month,
                <int>time::format(created_at, '%m') AS month_number
            FROM visitor_logs
            WHERE time::year(created_at) = $year
            GROUP BY month
            ORDER BY month_number ASC
        ";

        let vars: BTreeMap<String, Value> = [
            (
                "year".into(),
                year.into(),
            )
        ]
        .into();
       
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut visit_by_year_list: Vec<VisitByYear> = Vec::new();

        for object in into_iter_objects(responses)? {
            let visit_by_year_object = object?;

            let visit_by_year_model: Result<VisitByYear> = visit_by_year_object.try_into();
            visit_by_year_list.push(visit_by_year_model?);
        }

        println!("visit_by_year_list: {:?}", visit_by_year_list);
        // let visitor_log_model: Result<VistitorLogModel> = result_object?.try_into();
        
        Ok(visit_by_year_list)
    }

    
    /// Get visit by content type
    pub async fn get_visit_by_content_type(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        content_type: String,
        year: i64
    ) -> Result<Vec<VisitByContentType>> {

        let sql: &'static str = "
            SELECT  count() AS visits,  
                time::format(created_at, '%b') AS month,
                <int>time::format(created_at, '%m') AS month_number
            FROM visitor_logs
            WHERE content_type = $content_type and time::year(created_at) = $year
            GROUP BY month
            ORDER BY month_number ASC
        ";

        let vars: BTreeMap<String, Value> = [
            (
                "content_type".into(),
                content_type.into(),
            ),
            (
                "year".into(),
                year.into(),
            )
        ]
        .into();
       
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut visit_by_content_type_list: Vec<VisitByContentType> = Vec::new();

        for object in into_iter_objects(responses)? {
            let visit_by_year_object = object?;

            let visit_by_year_model: Result<VisitByContentType> = visit_by_year_object.try_into();
            visit_by_content_type_list.push(visit_by_year_model?);
        }

        // let visitor_log_model: Result<VistitorLogModel> = result_object?.try_into();
        
        Ok(visit_by_content_type_list)
    }

}