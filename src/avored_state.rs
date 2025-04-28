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
use crate::repositories::setting_repository::SettingRepository;
use crate::services::admin_user_service::AdminUserService;
use crate::services::asset_service::AssetService;
use crate::services::auth_service::AuthService;
use crate::services::content_service::ContentService;
use crate::services::misc_service::MiscService;
use crate::services::setting_service::SettingService;

pub struct AvoRedState {
    pub db: DB,
    pub config: AvoRedConfigProvider,
    pub template: AvoRedTemplateProvider,
    pub misc_service: MiscService,
    pub auth_service: AuthService,
    pub admin_user_service: AdminUserService,
    pub content_service: ContentService,
    pub asset_service: AssetService,
    pub setting_service: SettingService,
}

impl AvoRedState {
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


        let misc_service = MiscService::new().await?;
        let auth_service = AuthService::new(admin_user_repository.clone(), password_reset_repository.clone()).await?;
        let admin_user_service = AdminUserService::new(admin_user_repository, role_repository, password_reset_repository)?;
        let content_service = ContentService::new(content_repository, collection_repository)?;
        let asset_service = AssetService::new(asset_repository)?;
        let setting_service = SettingService::new(setting_repository)?;

        Ok(AvoRedState {
            config: avored_config_provider,
            template: avored_template_provider,
            db: avored_database_provider.db,
            misc_service,
            auth_service,
            admin_user_service,
            content_service,
            asset_service,
            setting_service
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
