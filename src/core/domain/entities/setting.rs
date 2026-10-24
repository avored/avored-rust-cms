use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingModel {
    pub id: String,
    pub identifier: String,
    pub value: String,
    pub setting_group: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorableSetting {
    pub identifier: String,
    pub value: String,
    pub setting_group: String,
}
