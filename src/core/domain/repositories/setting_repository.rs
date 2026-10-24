use crate::core::domain::entities::setting::{SettingModel, StorableSetting};
use crate::error::Result;

#[async_trait::async_trait]
pub trait SettingRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<SettingModel>>;
    async fn find_by_group(&self, group: &str) -> Result<Vec<SettingModel>>;
    async fn find_by_identifier(&self, identifier: &str) -> Result<Option<SettingModel>>;
    async fn upsert(&self, setting: StorableSetting) -> Result<SettingModel>;
}
