use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::component_model::{ComponentModel, CreatableComponent, PutComponentIdentifierModel, UpdatableComponentModel};
use crate::models::ModelCount;
use crate::PER_PAGE;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};
use super::into_iter_objects;

#[derive(Clone)]
pub struct ComponentRepository {}

impl ComponentRepository {
    pub fn new() -> Self {
        ComponentRepository {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
        order_column: String,
        order_type: String
    ) -> Result<Vec<ComponentModel>> {
        let sql = format!("\
            SELECT * \
            FROM components \
            ORDER {} {} \
            LIMIT $limit \
            START $start;\
        ", order_column, order_type);
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);
        let responses = datastore.execute(&sql, database_session, Some(vars)).await?;

        let mut component_list: Vec<ComponentModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let component_object = object?;

            let component_model: Result<ComponentModel> = component_object.try_into();
            component_list.push(component_model?);
        }
        Ok(component_list)
    }

    pub async fn all(
        &self,
        datastore: &Datastore,
        database_session: &Session
    ) -> Result<Vec<ComponentModel>> {
        let sql = "SELECT *, ->component_field->fields.* as fields FROM components";

        let responses = datastore.execute(sql, database_session, None).await?;

        let mut component_list: Vec<ComponentModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let component_object = object?;

            let component_model: Result<ComponentModel> = component_object.try_into();
            component_list.push(component_model?);
        }
        Ok(component_list)
    }

    pub async fn create_component(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_component_model: CreatableComponent,
    ) -> Result<ComponentModel> {

        let mut element_sql = String::from("");

        for element in creatable_component_model.elements {
            element_sql.push_str(
                &format!("{open_brace} \
                                name: '{name}', \
                                {close_brace}",
                         open_brace = String::from("{"),
                         name =  element.name,
                         close_brace = String::from("}")
                ));
        }

        let sql = format!("\
            CREATE components \
                CONTENT {open_brace} \
                    name: '{name}',
                    identifier: '{identifier}',
                    elements: [{element_sql}],
                    created_by: '{logged_in_user_email}',
                    updated_by: '{logged_in_user_email}',
                    created_at: time::now(),
                    updated_at: time::now(),
                {close_brace}",
            open_brace = String::from("{"),
            name = creatable_component_model.name,
            identifier = creatable_component_model.identifier,
            element_sql = element_sql,
            logged_in_user_email = creatable_component_model.logged_in_username,
            close_brace = String::from("}"));

        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let component_model: Result<ComponentModel> = result_object?.try_into();

        component_model
    }

    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        component_id: String,
    ) -> Result<ComponentModel> {
        let sql =
            "SELECT *, ->component_field->fields.* as fields FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), component_id.into()),
            ("table".into(), "components".into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        let component_model: Result<ComponentModel> = result_object?.try_into();

        component_model
    }

    pub async fn update_component(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_component_model: UpdatableComponentModel,
    ) -> Result<ComponentModel> {

        let mut element_sql = String::from("");

        for element in updatable_component_model.elements {
            element_sql.push_str(
                &format!("{open_brace} \
                                name: '{name}', \
                                {close_brace}",
                         open_brace = String::from("{"),
                         name =  element.name,
                         close_brace = String::from("}")
                ));
        }

        let sql = format!("\
            UPDATE components \
                MERGE {open_brace} \
                    name: '{name}',
                    elements: [{element_sql}],
                    updated_by: '{logged_in_user_email}',
                    updated_at: time::now(),
                {close_brace}",
                          open_brace = String::from("{"),
                          name = updatable_component_model.name,
                          element_sql = element_sql,
                          logged_in_user_email = updatable_component_model.logged_in_username,
                          close_brace = String::from("}"));


        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let component_model: Result<ComponentModel> = result_object?.try_into();

        component_model
    }

    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM components GROUP ALL;";
        let responses = datastore.execute(sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        match result_object {
            Ok(obj) => obj.try_into(),
            Err(_) => Ok(ModelCount::default()),
        }
    }

    pub async fn count_of_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        identifier: String
    ) -> Result<ModelCount> {
        let sql = "SELECT count(identifier=$identifier) FROM components GROUP ALL";

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

    pub async fn update_component_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        put_component_identifier_model: PutComponentIdentifierModel
    ) -> Result<ComponentModel> {
        let sql = "UPDATE type::thing($table, $id)
                    SET
                        identifier = $identifier,
                        updated_at = $updated_at,
                        updated_by = $updated_by
                    ;
        ";

        let vars: BTreeMap<String, Value> = [
            ("identifier".into(), put_component_identifier_model.identifier.into()),
            ("table".into(), "components".into()),
            ("updated_at".into(), Datetime::default().into()),
            ("updated_by".into(), put_component_identifier_model.logged_in_username.into()),
            ("id".into(), put_component_identifier_model.id.into())
        ].into();
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let updated_model: Result<ComponentModel> = result_object?.try_into();

        updated_model
    }
}
