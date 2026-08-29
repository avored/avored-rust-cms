use crate::error::{Result, Error};
pub mod auth_repository;


pub use auth_repository::AuthRepositoryImpl;
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
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(Error::Generic("empty object".to_string())),
            });

            Ok(it)
        }
        _ => Err(Error::Generic("No Record found".to_string())),
    }
}
