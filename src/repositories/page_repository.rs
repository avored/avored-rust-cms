use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};

use crate::error::{Error, Result};
use crate::models::page_model::{CreatablePageModel, PageModel, PutPageIdentifierModel, UpdatablePageModel};
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
        order_column: String,
        order_type: String,
    ) -> Result<Vec<PageModel>> {
        let sql = format!("\
            SELECT * \
            FROM type::table($table) \
            ORDER {} {}
            LIMIT $limit \
            START $start;\
        ", order_column, order_type);
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
            ("table".into(), PAGE_TABLE.into()),
        ]);
        let responses = datastore.execute(&sql, database_session, Some(vars)).await?;

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

    pub async fn remove_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        page_id: &String) -> Result<bool>
    {
        let sql = format!("DELETE pages:{page_id}");
        let responses = datastore.execute(&sql, database_session, None).await?;
        let response = responses
            .into_iter()
            .next()
            .map(|rp| rp.output());
        let query_result = match response {
            Some(object) => object.is_ok(), // there is another method is_err() as well
            None => false
        };
        Ok(query_result)
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
        logged_in_user: LoggedInUser,
    ) -> Result<PageModel> {
        let mut components_content_sql = String::from("");

        //@todo skip the last loop with comma think of how to make a comma and skip the last one.
        for creatable_component_content_model in creatable_page_model.component_contents {
            let mut elements_sql = String::from("");

            for creatable_component_element_content in creatable_component_content_model.elements {
                let mut element_data_sql = String::from("");

                for element_data_value in creatable_component_element_content.element_data {
                    element_data_sql.push_str(
                        &format!("{open_brace} \
                                label: '{label}', \
                                value: '{value}' \
                                {close_brace},",
                                 open_brace = String::from("{"),
                                 label = element_data_value.label,
                                 value = element_data_value.value,
                                 close_brace = String::from("},")
                        ));
                }

                elements_sql.push_str(
                    &format!("{open_brace} \
                        name: '{name}', \
                        identifier: '{identifier}', \
                        element_type: '{element_type}', \
                        element_data_type: '{element_data_type}', \
                        element_content: '{element_content}', \
                        element_data: [{element_data_sql}]  \
                        {close_brace},",
                             name = creatable_component_element_content.name,
                             identifier = creatable_component_element_content.identifier,
                             element_type = creatable_component_element_content.element_type,
                             element_data_type = creatable_component_element_content.element_data_type,
                             element_data_sql = element_data_sql,
                             element_content = creatable_component_element_content.element_content,
                             open_brace = String::from("{"),
                             close_brace = String::from("}")
                    ));
            }

            components_content_sql.push_str(
                &format!("{open_brace} \
                        name: '{name}', \
                        identifier: '{identifier}', \
                        elements: [{elements_sql}]  \
                        {close_brace},",
                         name = creatable_component_content_model.name,
                         identifier = creatable_component_content_model.identifier,
                         elements_sql = elements_sql,
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
        logged_in_user: LoggedInUser,
    ) -> Result<PageModel> {
        let mut components_content_sql = String::from("");

        //@todo skip the last loop with comma think of how to make a comma and skip the last one.
        for updatable_component_content_model in updatable_admin_user.component_contents {
            let mut elements_sql = String::from("");

            for updatable_component_element_content in updatable_component_content_model.elements {
                let mut element_data_sql = String::from("");

                for element_data_value in updatable_component_element_content.element_data {
                    element_data_sql.push_str(
                        &format!("{open_brace} \
                                label: '{label}', \
                                value: '{value}' \
                                {close_brace},",
                                 open_brace = String::from("{"),
                                 label = element_data_value.label,
                                 value = element_data_value.value,
                                 close_brace = String::from("},")
                        ));
                }

                elements_sql.push_str(&format!("{open_brace} \
                        name: '{name}', \
                        identifier: '{identifier}', \
                        element_type: '{element_type}', \
                        element_data_type: '{element_data_type}', \
                        element_content: '{element_content}', \
                        element_data: [{element_data_sql}\
                   ]  {close_brace},",
                       name = updatable_component_element_content.name,
                       identifier = updatable_component_element_content.identifier,
                       element_type = updatable_component_element_content.element_type,
                       element_content = updatable_component_element_content.element_content,
                       element_data_type = updatable_component_element_content.element_data_type,
                       element_data_sql = element_data_sql,
                       open_brace = String::from("{"),
                       close_brace = String::from("}")
                ));
            }

            components_content_sql.push_str(&format!("{open_brace} \
                    name: '{name}', \
                    identifier: '{identifier}', \
                    elements: [{elements_sql}]  \
                {close_brace},",
                     name = updatable_component_content_model.name,
                     identifier = updatable_component_content_model.identifier,
                     elements_sql = elements_sql,
                     open_brace = String::from("{"),
                     close_brace = String::from("}")
            ));
        }

        let sql = format!("
                UPDATE pages:{page_id} MERGE {open_brace}
                    name: '{name}',
                    components_content: [{components_content_sql}],
                    updated_by: '{logged_in_user_email}',
                    updated_at: time::now(),
                {close_brace};
            ",
                          page_id = updatable_admin_user.id,
                          name = updatable_admin_user.name,
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


    pub async fn count_of_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        identifier: String,
    ) -> Result<ModelCount> {
        let sql = "SELECT count(identifier=$identifier) FROM pages GROUP ALL";

        let vars: BTreeMap<String, Value> = [("identifier".into(), identifier.into())].into();
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }

    pub async fn update_page_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        put_page_identifier_model: PutPageIdentifierModel,
    ) -> Result<PageModel> {
        let sql = "UPDATE type::thing($table, $id)
                    SET
                        identifier = $identifier,
                        updated_at = $updated_at,
                        updated_by = $updated_by
                    ;
        ";

        let vars: BTreeMap<String, Value> = [
            ("identifier".into(), put_page_identifier_model.identifier.into()),
            ("table".into(), "pages".into()),
            ("updated_at".into(), Datetime::default().into()),
            ("updated_by".into(), put_page_identifier_model.logged_in_username.into()),
            ("id".into(), put_page_identifier_model.id.into())
        ].into();
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let updated_model: Result<PageModel> = result_object?.try_into();

        updated_model
    }
}
