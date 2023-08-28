use std::collections::BTreeMap;
use surrealdb::dbs::{Response, Session};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Array, Object};
use crate::models::role_model::RoleModel;
use crate::models::{W, ModelCount};
use crate::PER_PAGE;
use crate::error::Result;

pub struct RoleRepository {}

impl RoleRepository {
    pub fn new() -> RoleRepository {
        RoleRepository {}
    }

    pub async fn paginate(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        start: i64
    ) -> Result<Vec<RoleModel>> {
        let sql = "SELECT * FROM roles LIMIT $limit START $start;";
        let vars = BTreeMap::from([
            ("limit".into(), PER_PAGE.into()),
            ("start".into(), start.into()),
        ]);

        let responses = match datastore.execute(sql, &database_session, Some(vars), false).await {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response.result.expect(
            "there is an issue with receiving the response result of surreal db query response",
        );

        let array: Array = W(result)
            .try_into()
            .expect("there is an issue while converting query result into an array");
        let objects: Result<Vec<Object>> =
            array.into_iter().map(|value| W(value).try_into()).collect();
        let objects = match objects {
            Ok(obj) => obj,
            Err(_) => {
                let objects: Vec<Object> = vec![];

                objects
            }
        };

        let result_roles: Result<Vec<RoleModel>> = objects
            .into_iter()
            .map(|o|  o.try_into())
            .collect();

        let roles = match result_roles {
            Ok(data) => data,
            Err(_) => panic!("issue while converting an vector of objects into roles struct"),
        };

        Ok(roles)
    }

    pub async fn has_identifier_taken(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        identifier: String,
    ) -> Result<ModelCount> {
        let sql = "SELECT count() FROM roles WHERE identifier=$identifier;";
        let vars = BTreeMap::from([("identifier".into(), identifier.into())]);

        let responses = match datastore
            .execute(sql, &database_session, Some(vars), true)
            .await
        {
            Ok(response) => response,
            Err(_) => {
                let out: Vec<Response> = vec![];
                out
            }
        };

        let response = responses
            .into_iter()
            .next()
            .expect("there is an issue with unwrapping the surrealdb response");

        let result = response
            .result
            .expect(
                "there is an issue with receiving the response result of surreal db query response",
            )
            .first();
        println!("Result: {:?}", result);
        let result_object: Result<Object> = W(result).try_into();

        let role_count: Result<ModelCount> = match result_object {
            Ok(data) => data.try_into(),
            Err(_) => Ok(ModelCount::new()),
        };

        role_count
    }

}
