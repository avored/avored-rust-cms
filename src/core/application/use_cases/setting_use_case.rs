use std::collections::HashMap;

use crate::core::application::dtos::setting_dto::{
    EmailSettingsDto, GeneralSettingsDto, SeoSettingsDto,
    SettingsGroupResponse, UpdateSettingsPayload,
};
use crate::core::domain::entities::setting::StorableSetting;
use crate::core::domain::repositories::setting_repository::SettingRepository;
use crate::error::Result;

#[derive(Clone)]
pub struct SettingUseCase<R: SettingRepository> {
    repo: R,
}

impl<R: SettingRepository> SettingUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all_settings_grouped(&self) -> Result<SettingsGroupResponse> {
        let settings = self.repo.find_all().await?;
        let mut map: HashMap<String, String> = HashMap::new();
        for s in settings {
            map.insert(s.identifier, s.value);
        }

        let general = GeneralSettingsDto {
            site_name: map
                .get("general.site_name")
                .cloned()
                .unwrap_or_else(|| "AvoRed CMS".to_string()),
            logo_url: map
                .get("general.logo_url")
                .cloned()
                .unwrap_or_else(|| "/assets/images/avored.svg".to_string()),
            support_email: map
                .get("general.support_email")
                .cloned()
                .unwrap_or_else(|| "admin@avored.local".to_string()),
            timezone: map
                .get("general.timezone")
                .cloned()
                .unwrap_or_else(|| "UTC".to_string()),
            default_language: map
                .get("general.default_language")
                .cloned()
                .unwrap_or_else(|| "en".to_string()),
        };

        let seo = SeoSettingsDto {
            meta_title: map
                .get("seo.meta_title")
                .cloned()
                .unwrap_or_else(|| "AvoRed - Open Source Rust CMS".to_string()),
            meta_description: map
                .get("seo.meta_description")
                .cloned()
                .unwrap_or_else(|| "A modular, blazing fast CMS built with Rust.".to_string()),
            og_image_url: map
                .get("seo.og_image_url")
                .cloned()
                .unwrap_or_default(),
        };

        let email = EmailSettingsDto {
            smtp_host: map
                .get("email.smtp_host")
                .cloned()
                .unwrap_or_else(|| "smtp.mailtrap.io".to_string()),
            smtp_port: map
                .get("email.smtp_port")
                .cloned()
                .unwrap_or_else(|| "2525".to_string()),
            sender_name: map
                .get("email.sender_name")
                .cloned()
                .unwrap_or_else(|| "AvoRed CMS".to_string()),
            default_reply_to: map
                .get("email.default_reply_to")
                .cloned()
                .unwrap_or_else(|| "no-reply@avored.local".to_string()),
        };

        Ok(SettingsGroupResponse {
            general,
            seo,
            email,
        })
    }

    pub async fn update_settings(&self, payload: UpdateSettingsPayload) -> Result<SettingsGroupResponse> {
        let entries = vec![
            ("general.site_name", payload.general.site_name, "general"),
            ("general.logo_url", payload.general.logo_url, "general"),
            ("general.support_email", payload.general.support_email, "general"),
            ("general.timezone", payload.general.timezone, "general"),
            ("general.default_language", payload.general.default_language, "general"),
            ("seo.meta_title", payload.seo.meta_title, "seo"),
            ("seo.meta_description", payload.seo.meta_description, "seo"),
            ("seo.og_image_url", payload.seo.og_image_url, "seo"),
            ("email.smtp_host", payload.email.smtp_host, "email"),
            ("email.smtp_port", payload.email.smtp_port, "email"),
            ("email.sender_name", payload.email.sender_name, "email"),
            ("email.default_reply_to", payload.email.default_reply_to, "email")
        ];

        for (ident, val, grp) in entries {
            self.repo
                .upsert(StorableSetting {
                    identifier: ident.to_string(),
                    value: val,
                    setting_group: grp.to_string(),
                })
                .await?;
        }

        self.get_all_settings_grouped().await
    }
}
