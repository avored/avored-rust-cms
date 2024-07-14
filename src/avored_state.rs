use crate::error::Result;
use crate::providers::avored_config_provider::AvoRedConfigProvider;
use crate::providers::avored_database_provider::{AvoRedDatabaseProvider, DB};
// use crate::providers::avored_graphql_provider::{AvoRedGraphqlContext, AvoRedGraphqlProvider, AvoRedGraphqlSchema};
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::repositories::component_repository::ComponentRepository;
use crate::repositories::field_repository::FieldRepository;
use crate::repositories::page_repository::PageRepository;
use crate::repositories::role_repository::RoleRepository;
use crate::services::admin_user_service::AdminUserService;
use crate::services::component_service::ComponentService;
use crate::services::field_service::FieldService;
use crate::services::page_service::PageService;
use crate::services::role_service::RoleService;
use crate::repositories::asset_repository::AssetRepository;
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::repositories::setting_repository::SettingRepository;
use crate::services::asset_service::AssetService;
use crate::services::setting_service::SettingService;

pub struct AvoRedState {
    pub config: AvoRedConfigProvider,
    // pub schema: AvoRedGraphqlSchema,
    // pub context: AvoRedGraphqlContext,
    pub template: AvoRedTemplateProvider,
    pub db: DB,
    pub admin_user_service: AdminUserService,
    pub role_service: RoleService,
    pub component_service: ComponentService,
    pub field_service: FieldService,
    pub page_service: PageService,
    pub asset_service: AssetService,
    pub setting_service: SettingService
}

impl juniper::Context for AvoRedState{}

impl AvoRedState {
    pub async fn new() -> Result<AvoRedState> {
        // let avored_graphql_provider = AvoRedGraphqlProvider::register().await?;
        let avored_config_provider = AvoRedConfigProvider::register()?;
        let avored_database_provider =
            AvoRedDatabaseProvider::register(avored_config_provider.clone()).await?;

        let avored_template_provider = AvoRedTemplateProvider::register(
            avored_config_provider.clone()
        ).await?;

        let admin_user_repository = AdminUserRepository::new();
        let role_repository = RoleRepository::new();
        let component_repository = ComponentRepository::new();
        let field_repository = FieldRepository::new();
        let page_repository = PageRepository::new();
        let asset_repository = AssetRepository::new();
        let password_reset_repository = PasswordResetRepository::new();
        let setting_repository = SettingRepository::new();

        let admin_user_service = AdminUserService::new(admin_user_repository, role_repository.clone(), password_reset_repository.clone())?;
        let role_service = RoleService::new(role_repository)?;
        let component_service = ComponentService::new(component_repository)?;
        let field_service = FieldService::new(field_repository)?;
        let page_service = PageService::new(page_repository)?;
        let asset_service = AssetService::new(asset_repository)?;
        let setting_service = SettingService::new(setting_repository)?;

        Ok(AvoRedState {
            config: avored_config_provider,
            template: avored_template_provider,
            db: avored_database_provider.db,
            admin_user_service,
            role_service,
            component_service,
            field_service,
            page_service,
            asset_service,
            setting_service
        })
    }
}
