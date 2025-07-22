use crate::error::Error;
use crate::models::password_rest_model::{CreatablePasswordResetModel, PasswordResetModel};
use crate::repositories::into_iter_objects;
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{Datetime, Value};

const PASSWORD_RESET_TABLE: &str = "password_reset";

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
        let sql = "CREATE type::table($table) CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("email".into(), creatable_password_reset_model.email.into()),
            ("token".into(), creatable_password_reset_model.token.into()),
            ("status".into(), "Active".into()),
            ("created_at".into(), Datetime::default().into()),
            ("table".into(), PASSWORD_RESET_TABLE.into()),
        ]
        .into();

        let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let password_reset_model: crate::error::Result<PasswordResetModel> =
            result_object?.try_into();

        password_reset_model
    }

    pub async fn get_password_reset_by_email_and_token(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        email: &str,
        token: &str,
    ) -> crate::error::Result<PasswordResetModel> {
        let sql = "SELECT * FROM type::table($table) WHERE email=$email and token=$password_token";
        let vars: BTreeMap<String, Value> = [
            ("password_token".into(), token.into()),
            ("email".into(), email.into()),
            ("status".into(), "Active".into()),
            ("table".into(), PASSWORD_RESET_TABLE.into()),
        ]
        .into();

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic(format!(
                "no record found to reset password for email {email}"
            ))),
        };
        let password_reset_model: crate::error::Result<PasswordResetModel> =
            result_object?.try_into();

        password_reset_model
    }

    pub async fn expire_password_token_by_email(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        email: &str,
    ) -> crate::error::Result<bool> {
        let sql = "
            UPDATE type::table($table) SET status=$status WHERE email=$email";

        let vars = BTreeMap::from([
            ("status".into(), "Expired".into()),
            ("email".into(), email.into()),
            ("table".into(), PASSWORD_RESET_TABLE.into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        if result_object.is_ok() {
            return Ok(true);
        }

        Err(Error::Generic(format!(
            "issue while expiring the password token: {email}"
        )))
    }

    pub async fn expire_password_token_by_email_and_token(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        email: &str,
        token: &str,
    ) -> crate::error::Result<bool> {
        let sql = "
            UPDATE type::table($table) SET status=$status WHERE email=$email and token=$password_token";

        let vars = BTreeMap::from([
            ("status".into(), "Expired".into()),
            ("email".into(), email.into()),
            ("password_token".into(), token.into()),
            ("table".into(), PASSWORD_RESET_TABLE.into()),
        ]);

        let responses = datastore.execute(sql, database_session, Some(vars)).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };

        if result_object.is_ok() {
            return Ok(true);
        }

        Err(Error::Generic(format!(
            "issue while updating password by email: {email}"
        )))
    }
}
