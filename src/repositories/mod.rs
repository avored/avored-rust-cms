use crate::error::{Error, Result};
use surrealdb::dbs::Response;
use surrealdb::sql::{Object, Value};

/// admin user repository
pub mod admin_user_repository;

/// asset repository
pub mod asset_repository;

/// collection repository
pub mod collection_repository;


/// content repository
pub mod content_repository;

/// passwod reset repository
pub mod password_reset_repository;

/// role repository
pub mod role_repository;


/// security alert repository
pub mod security_alert_repository;


/// security audit repository
pub mod security_audit_repository;


/// setting repository
pub mod setting_repository;

/// visitor log repository module
pub mod visitor_log_repository;


/// into iter object
pub fn into_iter_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
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
