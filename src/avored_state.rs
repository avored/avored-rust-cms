use surrealdb::{dbs::Session, kvs::Datastore};

use crate::{config::Config, repositories::admin_user_repository::AdminUserRepository};

pub struct AvoRedState {
    pub datastore: Datastore,
    pub database_session: Session,
    pub admin_user_repository: AdminUserRepository
}

impl AvoRedState {
    pub async fn new(config: Config) -> AvoRedState {

        let datastore = Datastore::new("file://data/avored.db")
            .await
            .expect("there is issue with connecting with data/avored.db storage");

        let database_session = Session::for_db(
            config.database_namespace.clone(),
            config.database_name.clone(),
        );
        let admin_user_repository = AdminUserRepository::new();
        
        AvoRedState {
            datastore,
            database_session,
            admin_user_repository
        }
    }
}
