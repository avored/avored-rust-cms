use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};
use crate::error::Error;
use crate::models::password_rest_model::{CreatablePasswordResetModel, PasswordResetModel};
use crate::repositories::into_iter_objects;

#[derive(Clone)]
pub struct PasswordResetRepository {}

impl PasswordResetRepository {
    pub fn new() -> Self {
        PasswordResetRepository {}
    }

    pub async fn create_password_reset(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_password_reset_model: CreatablePasswordResetModel,
    ) -> crate::error::Result<PasswordResetModel> {
        let sql = "CREATE password_reset CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("email".into(), creatable_password_reset_model.email.into()),
            ("token".into(), creatable_password_reset_model.token.into()),
            ("created_at".into(), Datetime::default().into()),
        ]
            .into();
        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let password_reset_model: crate::error::Result<PasswordResetModel> = result_object?.try_into();

        password_reset_model
    }

    pub async fn get_password_reset_by_email(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        email: String,
    ) -> crate::error::Result<PasswordResetModel> {
        let sql = "SELECT * FROM password_reset WHERE email=$email";
        let vars: BTreeMap<String, Value> = [("email".into(), email.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let password_reset_model: crate::error::Result<PasswordResetModel> = result_object?.try_into();

        password_reset_model
    }
}
