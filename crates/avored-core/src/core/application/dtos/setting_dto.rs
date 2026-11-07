use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralSettingsDto {
    pub site_name: String,
    pub logo_url: String,
    pub support_email: String,
    pub timezone: String,
    pub default_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeoSettingsDto {
    pub meta_title: String,
    pub meta_description: String,
    pub og_image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailSettingsDto {
    pub smtp_host: String,
    pub smtp_port: String,
    pub sender_name: String,
    pub default_reply_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsGroupResponse {
    pub general: GeneralSettingsDto,
    pub seo: SeoSettingsDto,
    pub email: EmailSettingsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingsPayload {
    pub general: GeneralSettingsDto,
    pub seo: SeoSettingsDto,
    pub email: EmailSettingsDto,
}
