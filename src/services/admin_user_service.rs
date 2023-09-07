use std::collections::BTreeMap;

use surrealdb::{
    dbs::Response,
    sql::{Object, Value},
};

use crate::{
    error::{Error, Result},
    models::admin_user_model::AdminUserModel,
    providers::avored_database_provider::DB,
};

pub struct AdminUserService {}

impl AdminUserService {
    pub fn new() -> Result<Self> {
        Ok(AdminUserService {})
    }
}
impl AdminUserService {
    pub async fn all_admin_users(
        &self,
        (datastore, database_session): &DB,
    ) -> Result<Vec<AdminUserModel>> {
        let sql = "SELECT * FROM admin_users";

        let responses = datastore.execute(sql, database_session, None).await?;

        // println!("{:?}", responses);
        let mut admin_user_list: Vec<AdminUserModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let admin_user_object = object?;

            let admin_user_model: Result<AdminUserModel> = admin_user_object.try_into();
            admin_user_list.push(admin_user_model?);
        }
        // let task_model: Result<AdminUserModel> =
        Ok(admin_user_list)
    }

    pub async fn create_admin_user(
        &self,
        (ds, ses): &DB,
        email: &str,
        password: &str,
    ) -> Result<String> {
        let sql = "CREATE admin_users CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("email".into(), email.into()),
            ("password".into(), password.into()),
        ]
        .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let ress = ds.execute(sql, ses, Some(vars)).await?;

        into_iter_objects(ress)?
            .next()
            .transpose()?
            .and_then(|obj| obj.get("id").map(|id| id.to_string()))
            .ok_or_else(|| Error::Generic("no record"))
    }
}

fn into_iter_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let response = responses
        .into_iter()
        .next()
        .map(|rp| rp.result)
        .transpose()?;

    // println!("{:?}", response);

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
