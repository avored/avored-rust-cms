use std::collections::BTreeMap;
use surrealdb::sql::{Datetime, Value};
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use crate::PER_PAGE;
use crate::error::{Error, Result};
use crate::models::ModelCount;
use crate::models::entity_model::{CreatableEntityModel, EntityModel, PutEntityIdentifierModel, UpdatableEntityModel};
use crate::repositories::into_iter_objects;



const ENTITY_TABLE: &str = "entities";


/// admin user repository
#[derive(Clone)]
pub struct EntityRepository {}

impl Default for EntityRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityRepository {
    /// new instance for admin user repository
    #[must_use] pub const fn new() -> Self {
        Self {}
    }

        /// create entity
    pub async fn create_entity(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_entity_model: CreatableEntityModel,
    ) -> Result<EntityModel> {
        let sql = format!("CREATE {} CONTENT $data", ENTITY_TABLE);

        let data: BTreeMap<String, Value> = [
            (
                "name".into(),
                creatable_entity_model.name.into(),
            ),
            ("identifier".into(), creatable_entity_model.identifier.into()),
            
            (
                "created_by".into(),
                creatable_entity_model.logged_in_username.clone().into(),
            ),
            (
                "updated_by".into(),
                creatable_entity_model.logged_in_username.into(),
            ),
            ("created_at".into(), Datetime::default().into()),
            ("updated_at".into(), Datetime::default().into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = datastore.execute(&sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(ress)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let admin_user_model: Result<EntityModel> = result_object?.try_into();

        admin_user_model
    }

    /// get total count
    pub async fn get_total_count(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> Result<ModelCount> {
        let sql = format!("SELECT count() FROM {} GROUP ALL;", ENTITY_TABLE);
        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }

    /// entity paginate
    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64,
        order_column: String,
        order_type: String,
    ) -> Result<Vec<EntityModel>> {
        let sql = format!(
            "\
            SELECT * \
            FROM {} \
            ORDER {order_column} {order_type} \
            LIMIT $limit \
            START $start;\
        ", ENTITY_TABLE
        );
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
            ("order_type".into(), "id".into()),
        ]);

        let responses = datastore
            .execute(&sql, database_session, Some(vars))
            .await
            .unwrap();
        let mut model_list: Vec<EntityModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let model_object = object?;

            let entity_model: Result<EntityModel> = model_object.try_into();
            model_list.push(entity_model?);
        }

        Ok(model_list)
    }


    /// find by id entity 
    pub async fn find_by_id(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        id: &str,
    ) -> Result<EntityModel> {
        let sql = "SELECT * FROM type::thing($table, $id);";
        // let sql = "SELECT * FROM type::thing($table, $id);";
        let vars = BTreeMap::from([
            ("table".into(), ENTITY_TABLE.into()),
            ("id".into(), id.into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model: Result<EntityModel> = result_object?.try_into();

        model
    }

    /// update entity
    pub async fn update_entity(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_model: UpdatableEntityModel,
    ) -> Result<EntityModel> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                name: $name,
                updated_by: $logged_in_user_name,
                updated_at: time::now()
            };";

        let vars = BTreeMap::from([
            ("name".into(), updatable_model.name.into()),
            (
                "logged_in_user_name".into(),
                updatable_model.logged_in_username.into(),
            ),
            ("id".into(), updatable_model.id.into()),
            ("table".into(), ENTITY_TABLE.into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let admin_user_model: Result<EntityModel> = result_object?.try_into();

        admin_user_model
    }

    /// update entity identifier
    pub async fn update_entity_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        put_role_identifier_model: PutEntityIdentifierModel,
    ) -> Result<EntityModel> {
        let sql = "UPDATE type::thing($table, $id)
                    SET
                        identifier = $identifier,
                        updated_at = $updated_at,
                        updated_by = $updated_by
                    ;
        ";

        let vars: BTreeMap<String, Value> = [
            (
                "identifier".into(),
                put_role_identifier_model.identifier.into(),
            ),
            ("table".into(), ENTITY_TABLE.into()),
            ("updated_at".into(), Datetime::default().into()),
            (
                "updated_by".into(),
                put_role_identifier_model.logged_in_username.into(),
            ),
            ("id".into(), put_role_identifier_model.id.into()),
        ]
        .into();
        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let updated_model: Result<EntityModel> = result_object?.try_into();

        updated_model
    }

    /// count of identifier
    pub async fn count_of_identifier(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        identifier: &str,
    ) -> Result<ModelCount> {
        let sql = format!("SELECT count(identifier=$identifier) FROM {} GROUP ALL", ENTITY_TABLE);

        let vars: BTreeMap<String, Value> = [("identifier".into(), identifier.into())].into();
        let responses = datastore.execute(&sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let model_count: Result<ModelCount> = result_object?.try_into();

        model_count
    }

    /// delete entity
    pub async fn delete_entity(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        entity_id: &str,
    ) -> Result<bool> {
        let sql = "
            DELETE type::thing($table, $id);";

        let vars: BTreeMap<String, Value> = [
            ("id".into(), entity_id.into()),
            ("table".into(), ENTITY_TABLE.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;
        let response = responses.into_iter().next().map(|rp| rp.result).transpose();
        if response.is_ok() {
            return Ok(true);
        }

        Ok(false)
    }

    
}