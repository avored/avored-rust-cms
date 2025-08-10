use crate::error::Error;
use crate::models::setting_model::{SettingModel, UpdatableSettingModel};
use crate::repositories::into_iter_objects;
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;

/// setting repository
#[derive(Clone)]
pub struct SettingRepository {}

impl SettingRepository {
    /// new instance
    pub fn new() -> Self {
        SettingRepository {}
    }

    /// all settings
    pub async fn all(
        &self,
        datastore: &Datastore,
        database_session: &Session,
    ) -> crate::error::Result<Vec<SettingModel>> {
        let sql = "SELECT * FROM settings";

        let responses = datastore.execute(sql, database_session, None).await?;

        let mut settings_list: Vec<SettingModel> = Vec::new();

        for object in into_iter_objects(responses)? {
            let model_object = object?;

            let setting_model: crate::error::Result<SettingModel> = model_object.try_into();
            settings_list.push(setting_model?);
        }
        Ok(settings_list)
    }

    /// update settings
    pub async fn update_setting(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_setting: UpdatableSettingModel,
    ) -> crate::error::Result<bool> {
        let sql = "
            UPDATE type::thing($table, $id) MERGE {
                value: $value,
                updated_by: $logged_in_user_name,
                updated_at: time::now(),
            };";

        let vars = BTreeMap::from([
            ("table".into(), "settings".into()),
            ("value".into(), updatable_setting.value.into()),
            ("id".into(), updatable_setting.id.into()),
            (
                "logged_in_user_name".into(),
                updatable_setting.logged_in_username.into(),
            ),
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

        Ok(false)
    }

    // pub async fn find_by_identifier(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     identifier: String,
    // ) -> crate::error::Result<SettingModel> {
    //     let sql = "SELECT * FROM settings WHERE identifier=$data;";
    //     let data: BTreeMap<String, Value> = [("data".into(), identifier.into())].into();
    //
    //     let responses = datastore.execute(sql, database_session, Some(data)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found".to_string())),
    //     };
    //     let setting_model: crate::error::Result<SettingModel> = result_object?.try_into();
    //
    //     setting_model
    // }
}
