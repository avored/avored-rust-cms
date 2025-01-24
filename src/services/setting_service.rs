use crate::models::setting_model::{SettingModel, UpdatableSettingModel};
use crate::providers::avored_database_provider::DB;
use crate::{error::Result, repositories::setting_repository::SettingRepository};

pub struct SettingService {
    setting_repository: SettingRepository,
}

impl SettingService {
    pub fn new(setting_repository: SettingRepository) -> Result<Self> {
        Ok(SettingService { setting_repository })
    }

    pub async fn all(&self, (datastore, database_session): &DB) -> Result<Vec<SettingModel>> {
        self.setting_repository
            .all(datastore, database_session)
            .await
    }

    pub async fn find_by_identifier(
        &self,
        (datastore, database_session): &DB,
        identifier: String,
    ) -> Result<SettingModel> {
        self.setting_repository
            .find_by_identifier(datastore, database_session, identifier)
            .await
    }

    pub async fn update_setting(
        &self,
        (datastore, database_session): &DB,
        updatable_setting: UpdatableSettingModel,
    ) -> Result<bool> {
        self.setting_repository
            .update_setting(datastore, database_session, updatable_setting)
            .await
    }
}
impl SettingService {}
