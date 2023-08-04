use surrealdb::{dbs::Session, kvs::Datastore};
use handlebars::Handlebars;

use crate::{config::Config, repositories::admin_user_repository::AdminUserRepository, providers::{avored_view_provider::AvoRedViewProviders, avored_database_provider::AvoRedDatabaseProvider}};

pub struct AvoRedState {
    pub datastore: Datastore,
    pub database_session: Session,
    pub handlebars: Handlebars<'static>,
    pub admin_user_repository: AdminUserRepository
}

impl AvoRedState {
    pub async fn new(config: Config) -> AvoRedState {
        let avored_database_provider = AvoRedDatabaseProvider::register(config).await;
        let avored_view_provider = AvoRedViewProviders::register();
        let admin_user_repository = AdminUserRepository::new();
        
        AvoRedState {
            datastore: avored_database_provider.datastore,
            database_session: avored_database_provider.database_session,
            handlebars: avored_view_provider.handlebars,
            admin_user_repository,
        }
    }
}
