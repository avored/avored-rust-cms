use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::component_model::{ComponentModel, CreatableComponent, UpdatableComponentModel};
use crate::models::field_model::FieldModel;
use crate::models::ModelCount;
use crate::PER_PAGE;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};

use super::into_iter_objects;

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
    ) -> Result<Vec<ComponentModel>> {
        let sql = "SELECT * FROM components LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

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
        createable_component_model: CreatableComponent,
    ) -> Result<ComponentModel> {
        let sql = "CREATE components CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("name".into(), createable_component_model.name.into()),
            (
                "identifier".into(),
                createable_component_model.identifier.into(),
            ),
            // ("fields".into(), createable_component_model.fields.into()),
            (
                "created_by".into(),
                createable_component_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                createable_component_model.logged_in_username.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
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
            "SELECT *, ->component_field<-components.* as fields FROM type::thing($table, $id);";
        let vars: BTreeMap<String, Value> = [
            ("id".into(), component_id.into()),
            ("table".into(), "components".into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        // println!("RESULT_OBJECT: {result_object:?}");
        let component_model: Result<ComponentModel> = result_object?.try_into();

        component_model
    }

    pub async fn update_component(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_admin_user: UpdatableComponentModel,
    ) -> Result<ComponentModel> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                name: $name,
                identifier: $identifier,
                updated_by: $logged_in_user_name,
                updated_at: time::now()
            };";

        let vars = BTreeMap::from([
            ("name".into(), updatable_admin_user.name.into()),
            ("identifier".into(), updatable_admin_user.identifier.into()),
            (
                "logged_in_user_name".into(),
                updatable_admin_user.logged_in_username.into(),
            ),
            ("id".into(), updatable_admin_user.id.into()),
            ("table".into(), "components".into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found")),
        };
        let component_model: Result<ComponentModel> = result_object?.try_into();

        component_model
    }

    pub async fn delete_component(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        component_id: String,
    ) -> Result<bool> {
        let sql = "
            DELETE type::thing($table, $id);";

        let vars: BTreeMap<String, Value> = [
            ("id".into(), component_id.into()),
            ("table".into(), "components".into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;
        let response = responses.into_iter().next().map(|rp| rp.result).transpose();
        if response.is_ok() {
            return Ok(true);
        }

        Ok(false)
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
            None => Err(Error::Generic("no record found")),
        };

        let total_count = match result_object {
            Ok(obj) => obj.try_into(),
            Err(_) => Ok(ModelCount::default()),
        };

        total_count
    }

    pub async fn attach_component_with_field(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        component_model: ComponentModel,
        field_model: FieldModel,
        logged_in_username: String,
    ) -> Result<bool> {
        let sql = format!(
            "RELATE {}:{}->{}->{}:{} CONTENT $attached_data;",
            "components", component_model.id, "component_field", "fields", field_model.id
        );

        let attached_data: BTreeMap<String, Value> = [
            ("created_by".into(), logged_in_username.clone().into()),
            ("updated_by".into(), logged_in_username.into()),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();

        let vars: BTreeMap<String, Value> = [("attached_data".into(), attached_data.into())].into();

        let responses = datastore
            .execute(sql.as_str(), database_session, Some(vars))
            .await?;

        let response = responses.into_iter().next().map(|rp| rp.result).transpose();
        if response.is_ok() {
            return Ok(true);
        }

        Ok(true)
    }
}
