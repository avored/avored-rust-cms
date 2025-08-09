use crate::error::Result;
use crate::providers::avored_config_provider::AvoRedConfigProvider;
use crate::providers::avored_database_provider::{AvoRedDatabaseProvider, DB};
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::repositories::asset_repository::AssetRepository;
use crate::repositories::collection_repository::CollectionRepository;
use crate::repositories::content_repository::ContentRepository;
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::security_alert_repository::SecurityAlertRepository;
use crate::repositories::security_audit_repository::SecurityAuditRepository;
use crate::repositories::setting_repository::SettingRepository;
use crate::services::admin_user_service::AdminUserService;
use crate::services::asset_service::AssetService;
use crate::services::auth_service::AuthService;
use crate::services::cms_service::CmsService;
use crate::services::content_service::ContentService;
use crate::services::general_service::GeneralService;
use crate::services::misc_service::MiscService;
use crate::services::security_alert_service::SecurityAlertService;
use crate::services::security_audit_service::SecurityAuditService;
use crate::services::setting_service::SettingService;

/// AvoRedState holds the global state for the AvoRed application, including configuration,
pub struct AvoRedState {

    /// database connection, and various services.
    pub db: DB,

    /// Configuration provider for AvoRed.
    pub config: AvoRedConfigProvider,

    /// Template provider for AvoRed, used for rendering views.
    pub template: AvoRedTemplateProvider,

    /// Miscellaneous service for handling non-specific tasks.
    pub misc_service: MiscService,

    /// Authentication service for managing user authentication and authorization.
    pub auth_service: AuthService,

    /// Service for managing admin users, including roles and permissions.
    pub admin_user_service: AdminUserService,

    /// Service for managing content, including collections and individual content items.
    pub content_service: ContentService,

    /// Service for managing assets, such as files and images.
    pub asset_service: AssetService,

    /// Service for managing settings and configurations.
    pub setting_service: SettingService,

    /// Service for managing CMS-related functionalities.
    pub cms_service: CmsService,

    /// General service for handling common operations across the application.
    pub general_service: GeneralService,

    /// Service for handling security audits, logging security-related events.
    pub security_audit_service: SecurityAuditService,

    /// Service for handling security alerts, managing security notifications and responses.
    pub security_alert_service: SecurityAlertService,
}

impl AvoRedState {
    /// Creates a new instance of `AvoRedState`, initializing all providers and services.
    pub async fn new() -> Result<AvoRedState> {
        let avored_config_provider = AvoRedConfigProvider::register()?;
        let avored_template_provider =
            AvoRedTemplateProvider::register(avored_config_provider.clone()).await?;
        let avored_database_provider =
            AvoRedDatabaseProvider::register(avored_config_provider.clone()).await?;

        let admin_user_repository = AdminUserRepository::new();
        let role_repository = RoleRepository::new();
        let collection_repository = CollectionRepository::new();
        let content_repository = ContentRepository::new();
        let asset_repository = AssetRepository::new();
        let password_reset_repository = PasswordResetRepository::new();
        let setting_repository = SettingRepository::new();
        let security_audit_repository = SecurityAuditRepository::new();
        let security_alert_repository = SecurityAlertRepository::new();

        let misc_service = MiscService::new().await?;
        let auth_service =
            AuthService::new(admin_user_repository.clone(), password_reset_repository).await?;
        let admin_user_service = AdminUserService::new(admin_user_repository, role_repository)?;
        let content_service =
            ContentService::new(content_repository.clone(), collection_repository)?;
        let asset_service = AssetService::new(asset_repository)?;
        let setting_service = SettingService::new(setting_repository)?;
        let cms_service = CmsService::new(content_repository)?;
        let general_service = GeneralService::new()?;
        let security_audit_service = SecurityAuditService::new(security_audit_repository);
        let security_alert_service = SecurityAlertService::new(security_alert_repository);

        Ok(AvoRedState {
            config: avored_config_provider,
            template: avored_template_provider,
            db: avored_database_provider.db,
            misc_service,
            auth_service,
            admin_user_service,
            content_service,
            asset_service,
            setting_service,
            cms_service,
            general_service,
            security_audit_service,
            security_alert_service,
        })
    }
}

// use crate::error::Result;
// use crate::providers::avored_config_provider::AvoRedConfigProvider;
// use crate::providers::avored_database_provider::{AvoRedDatabaseProvider, DB};
// use crate::providers::avored_template_provider::AvoRedTemplateProvider;
// use crate::repositories::admin_user_repository::AdminUserRepository;
// use crate::repositories::asset_repository::AssetRepository;
// use crate::repositories::collection_repository::CollectionRepository;
// use crate::repositories::component_repository::ComponentRepository;
// use crate::repositories::content_repository::ContentRepository;
// use crate::repositories::model_repository::ModelRepository;
// use crate::repositories::page_repository::PageRepository;
// use crate::repositories::password_reset_repository::PasswordResetRepository;
// use crate::repositories::role_repository::RoleRepository;
// use crate::repositories::setting_repository::SettingRepository;
// use crate::services::admin_user_service::AdminUserService;
// use crate::services::asset_service::AssetService;
// use crate::services::cms_service::CmsService;
// use crate::services::collection_service::CollectionService;
// use crate::services::component_service::ComponentService;
// use crate::services::content_service::ContentService;
// use crate::services::model_service::ModelService;
// use crate::services::page_service::PageService;
// use crate::services::role_service::RoleService;
// use crate::services::setting_service::SettingService;
//
// pub struct AvoRedState {
//     pub config: AvoRedConfigProvider,
//     pub template: AvoRedTemplateProvider,
//     pub db: DB,
//     pub admin_user_service: AdminUserService,
//     pub role_service: RoleService,
//     pub component_service: ComponentService,
//     pub page_service: PageService,
//     pub asset_service: AssetService,
//     pub setting_service: SettingService,
//     pub model_service: ModelService,
//     pub cms_service: CmsService,
//     pub collection_service: CollectionService,
//     pub content_service: ContentService,
// }
//
// impl AvoRedState {
//     pub async fn new() -> Result<AvoRedState> {
//         let avored_config_provider = AvoRedConfigProvider::register()?;
//         let avored_database_provider =
//             AvoRedDatabaseProvider::register(avored_config_provider.clone()).await?;
//
//         let avored_template_provider =
//             AvoRedTemplateProvider::register(avored_config_provider.clone()).await?;
//
//         let model_repository = ModelRepository::new();
//         let admin_user_repository = AdminUserRepository::new();
//         let role_repository = RoleRepository::new();
//         let component_repository = ComponentRepository::new();
//         let page_repository = PageRepository::new();
//         let asset_repository = AssetRepository::new();
//         let password_reset_repository = PasswordResetRepository::new();
//         let setting_repository = SettingRepository::new();
//         let collection_repository = CollectionRepository::new();
//         let content_repository = ContentRepository::new();
//
//         let admin_user_service = AdminUserService::new(
//             admin_user_repository,
//             role_repository.clone(),
//             password_reset_repository.clone(),
//         )?;
//         let role_service = RoleService::new(role_repository)?;
//         let component_service = ComponentService::new(component_repository)?;
//         let page_service = PageService::new(page_repository)?;
//         let asset_service = AssetService::new(asset_repository)?;
//         let setting_service = SettingService::new(setting_repository)?;
//         let model_service = ModelService::new(model_repository)?;
//         let cms_service = CmsService::new()?;
//         let collection_service = CollectionService::new(collection_repository)?;
//         let content_service = ContentService::new(content_repository)?;
//
//         Ok(AvoRedState {
//             config: avored_config_provider,
//             template: avored_template_provider,
//             db: avored_database_provider.db,
//             admin_user_service,
//             role_service,
//             component_service,
//             page_service,
//             asset_service,
//             setting_service,
//             model_service,
//             cms_service,
//             collection_service,
//             content_service,
//         })
//     }
// }
