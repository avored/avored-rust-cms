use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;

use crate::error::{Error, Result};
use crate::models::page_model::{CreatablePageModel, PageModel, UpdatablePageModel};
use crate::models::ModelCount;
use crate::models::token_claim_model::LoggedInUser;
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
            None => Err(Error::Generic("no record found".to_string())),
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
            None => Err(Error::Generic("no record found".to_string())),
        };

        let page_model: Result<PageModel> = result_object?.try_into();

        page_model
    }

    pub async fn create_page(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_page_model: CreatablePageModel,
        logged_in_user: LoggedInUser
    ) -> Result<PageModel> {

        let mut components_content_sql = String::from("");

        //@todo skip the last loop with comma think of how to make a comma and skip the last one.
        for creatable_component_content_model in creatable_page_model.component_contents {

            let mut fields_sql = String::from("");

            for creatable_component_field_content in creatable_component_content_model.fields {

                fields_sql.push_str(&format!("{open_brace} id: '{id}', name: '{name}', identifier: '{identifier}', field_type: '{field_type}', field_content: '{field_content}'  {close_brace}",
                                                       id = creatable_component_field_content.id,
                                                       name = creatable_component_field_content.name,
                                                       identifier = creatable_component_field_content.identifier,
                                                       field_type = creatable_component_field_content.field_type,
                                                       field_content = creatable_component_field_content.field_content,
                                                       open_brace = String::from("{"),
                                                       close_brace = String::from("}")
                ));
            }

            components_content_sql.push_str(&format!("{open_brace} id: '{id}', name: '{name}', identifier: '{identifier}', fields: [{fields_sql}]  {close_brace}",
                                                id = creatable_component_content_model.id,
                                                name = creatable_component_content_model.name,
                                                identifier = creatable_component_content_model.identifier,
                                                 fields_sql = fields_sql,
                                                open_brace = String::from("{"),
                                                close_brace = String::from("}")
            ));
        }

        let sql = format!("
                CREATE pages CONTENT {open_brace}
                    name: '{name}',
                    identifier: '{identifier}',
                    components_content: [{components_content_sql}],
                    created_by: '{logged_in_user_email}',
                    updated_by: '{logged_in_user_email}',
                    created_at: time::now(),
                    updated_at: time::now(),
                {close_brace};
            ",
            name = creatable_page_model.name,
            identifier = creatable_page_model.identifier,
            components_content_sql = components_content_sql,
            logged_in_user_email = logged_in_user.email,
            open_brace = String::from("{"),
            close_brace = String::from("}")
        );
        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let page_model: Result<PageModel> = result_object?.try_into();

        page_model
    }

    pub async fn update_page(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_admin_user: UpdatablePageModel,
        logged_in_user: LoggedInUser
    ) -> Result<PageModel> {
        let mut components_content_sql = String::from("");

        //@todo skip the last loop with comma think of how to make a comma and skip the last one.
        for updatable_component_content_model in updatable_admin_user.component_contents {

            let mut fields_sql = String::from("");

            for updatable_component_field_content in updatable_component_content_model.fields {

                fields_sql.push_str(&format!("{open_brace} id: '{id}', name: '{name}', identifier: '{identifier}', field_type: '{field_type}', field_content: '{field_content}'  {close_brace}",
                                                               id = updatable_component_field_content.id,
                                                               name = updatable_component_field_content.name,
                                                               identifier = updatable_component_field_content.identifier,
                                                               field_type = updatable_component_field_content.field_type,
                                                               field_content = updatable_component_field_content.field_content,
                                                               open_brace = String::from("{"),
                                                               close_brace = String::from("}")
                ));
            }

            components_content_sql.push_str(&format!("{open_brace} id: '{id}', name: '{name}', identifier: '{identifier}', fields: [{fields_sql}]  {close_brace}",
                                                     id = updatable_component_content_model.id,
                                                     name = updatable_component_content_model.name,
                                                     identifier = updatable_component_content_model.identifier,
                                                     fields_sql = fields_sql,
                                                     open_brace = String::from("{"),
                                                     close_brace = String::from("}")
            ));
        }

        let sql = format!("
                UPDATE pages:{page_id} MERGE {open_brace}
                    name: '{name}',
                    identifier: '{identifier}',
                    components_content: [{components_content_sql}],
                    updated_by: '{logged_in_user_email}',
                    updated_at: time::now(),
                {close_brace};
            ",
                page_id = updatable_admin_user.id,
                name = updatable_admin_user.name,
                identifier = updatable_admin_user.identifier,
                components_content_sql = components_content_sql,
                logged_in_user_email = logged_in_user.email,
                open_brace = String::from("{"),
                close_brace = String::from("}")
        );

        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let page_model: Result<PageModel> = result_object?.try_into();

        page_model
    }



}
