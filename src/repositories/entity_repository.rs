use std::collections::BTreeMap;
use surrealdb::sql::{Datetime, Value};
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use crate::PER_PAGE;
use crate::error::{Error, Result};
use crate::models::ModelCount;
use crate::models::entity_model::{CreatableEntityModel, EntityModel};
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

    
}