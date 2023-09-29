use crate::error::{Error, Result};
use surrealdb::dbs::Response;
use surrealdb::sql::{Object, Value};

pub mod admin_user_repository;
pub mod component_repository;
pub mod field_repository;
pub mod page_repository;
pub mod role_repository;

fn into_iter_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let response = responses
        .into_iter()
        .next()
        .map(|rp| rp.result)
        .transpose()?;

    match response {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(Error::Generic("empty object")),
            });

            Ok(it)
        }
        _ => Err(Error::Generic("No Record found")),
    }
}
