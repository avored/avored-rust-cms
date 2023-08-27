use surrealdb::{dbs::Session, kvs::Datastore};
use handlebars::Handlebars;

use crate::{providers::avored_config_provider::AvoRedConfigProvider, repositories::admin_user_repository::AdminUserRepository, providers::{avored_view_provider::AvoRedViewProviders, avored_database_provider::AvoRedDatabaseProvider}};
use crate::services::role_service::RoleService;

pub struct AvoRedState {
    pub datastore: Datastore,
    pub database_session: Session,
    pub handlebars: Handlebars<'static>,
    pub admin_user_repository: AdminUserRepository,
    pub role_service: RoleService
}

impl AvoRedState {
    pub async fn new(config: AvoRedConfigProvider) -> AvoRedState {
        let avored_database_provider = AvoRedDatabaseProvider::register(config).await;
        let avored_view_provider = AvoRedViewProviders::register();
        let admin_user_repository = AdminUserRepository::new();
        let role_service = RoleService::new();
        
        AvoRedState {
            datastore: avored_database_provider.datastore,
            database_session: avored_database_provider.database_session,
            handlebars: avored_view_provider.handlebars,
            admin_user_repository,
            role_service

        }
    }
}
