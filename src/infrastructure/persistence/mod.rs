use crate::error::{Result, Error};
pub mod auth_repository;

pub mod misc_repository;
pub mod entity_repository;

pub use auth_repository::AuthRepositoryImpl;
pub use entity_repository::EntityRepositoryImpl;
use surrealdb::types::{Object, Value};
use surrealdb_core::dbs::QueryResult;



pub fn into_iter_objects(responses: Vec<QueryResult>) -> Result<impl Iterator<Item = Result<Object>>> {
    let response = responses
        .into_iter()
        .next()
        .map(|rp| rp.result)
        .transpose()?;

    match response {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().filter_map(|v| match v {
                Value::Object(object) => Some(Ok(object)),
                _ => None,
            });

            Ok(it)
        }
        _ => Err(Error::Generic("No Record found".to_string())),
    }
}
