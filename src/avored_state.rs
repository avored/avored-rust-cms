use crate::error::Result;
use crate::providers::avored_config_provider::AvoRedConfigProvider;
use crate::providers::avored_database_provider::{AvoRedDatabaseProvider, DB};
use crate::providers::avored_view_provider::AvoRedViewProvider;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::services::admin_user_service::AdminUserService;
use handlebars::Handlebars;

pub struct AvoRedState {
    pub handlebars: Handlebars<'static>,
    pub config: AvoRedConfigProvider,
    pub db: DB,
    pub admin_user_service: AdminUserService,
}

impl AvoRedState {
    pub async fn new() -> Result<AvoRedState> {
        let avored_view_provider = AvoRedViewProvider::register()?;
        let avored_config_provider = AvoRedConfigProvider::register()?;
        let avored_database_provider =
            AvoRedDatabaseProvider::register(avored_config_provider.clone()).await?;

        let admin_user_repository = AdminUserRepository::new();
        let admin_user_service = AdminUserService::new(admin_user_repository)?;

        Ok(AvoRedState {
            handlebars: avored_view_provider.handlebars,
            config: avored_config_provider,
            db: avored_database_provider.db,
            admin_user_service,
        })
    }
}
