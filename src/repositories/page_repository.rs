use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

use crate::error::{Error, Result};
use crate::models::page_model::{CreatablePageModel, PageModel, UpdatablePageModel};
use crate::models::ModelCount;
use crate::PER_PAGE;

use super::into_iter_objects;
const PAGE_TABLE: &str = "pages";

#[derive(Clone)]
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
        let sql = "SELECT * FROM type::table($table) LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
            ("table".into(), PAGE_TABLE.into()),
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

    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        page_id: String,
    ) -> Result<PageModel> {
        let sql =
            "SELECT * FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), page_id.into()),
            ("table".into(), "pages".into()),
        ]
            .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        // println!("RESULT_OBJECT: {result_object:?}");
        let page_model: Result<PageModel> = result_object?.try_into();

        page_model
    }

    pub async fn create_page(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_page_model: CreatablePageModel,
    ) -> Result<PageModel> {

        // println!("REPO MODEL: {creatable_page_model:?}");
        //
        // let sql = "CREATE pages CONTENT $data";
        //
        // let data: BTreeMap<String, Value> = [
        //     ("name".into(), creatable_page_model.name.into()),
        //     (
        //         "identifier".into(),
        //         creatable_page_model.identifier.into(),
        //     ),
        //     ("component_content".into(), creatable_page_model.component_content.into()),
        //     ("content".into(), creatable_page_model.content.into()),
        //     (
        //         "created_by".into(),
        //         creatable_page_model.logged_in_username.clone().into(),
        //     ),
        //     (
        //         "updated_by".into(),
        //         creatable_page_model.logged_in_username.into(),
        //     ),
        //     ("created_at".into(), Datetime::default().into()),
        //     ("updated_at".into(), Datetime::default().into()),
        // ]
        //     .into();
        // let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
        //
        // let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let mut component_content_sql = String::from("");
        for creatable_component_content_model in creatable_page_model.component_content {
            component_content_sql.push_str(&format!("{open_brace} id: '{id}', name: '{name}', identifier: '{identifier}', content: '{content}' {close_brace}",
                                                id = creatable_component_content_model.id,
                                                name = creatable_component_content_model.name,
                                                identifier = creatable_component_content_model.identifier,
                                                content =  creatable_component_content_model.content,
                                                open_brace = String::from("{"),
                                                close_brace = String::from("}")
            ));
        }

            let sql = format!("
                    CREATE pages CONTENT {open_brace}
                        name: '{name}',
                        identifier: '{identifier}',
                        component_content: [{component_content_sql}],
                        content: '',
                        created_by: 'admin@admin.com',
                        updated_by: 'admin@admin.com',
                        created_at: time::now(),
                        updated_at: time::now(),
                    {close_brace};
                ",
                name = creatable_page_model.name,
                identifier = creatable_page_model.identifier,
                component_content_sql = component_content_sql,
                open_brace = String::from("{"),
                close_brace = String::from("}")
            );

        println!("SQ: {sql}");

        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let page_model: Result<PageModel> = result_object?.try_into();

        page_model
    }

    pub async fn update_page(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_admin_user: UpdatablePageModel,
    ) -> Result<PageModel> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                name: $name,
                identifier: $identifier,
                content: $content,
                updated_by: $logged_in_user_name,
                updated_at: time::now()
            };";

        let vars = BTreeMap::from([
            ("name".into(), updatable_admin_user.name.into()),
            ("identifier".into(), updatable_admin_user.identifier.into()),
            ("content".into(), updatable_admin_user.content.into()),
            (
                "logged_in_user_name".into(),
                updatable_admin_user.logged_in_username.into(),
            ),
            ("id".into(), updatable_admin_user.id.into()),
            ("table".into(), "pages".into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let page_model: Result<PageModel> = result_object?.try_into();

        page_model
    }



}
