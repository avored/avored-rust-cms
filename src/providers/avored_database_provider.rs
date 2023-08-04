use surrealdb::{dbs::Session, kvs::Datastore};

use crate::config::Config;

pub struct AvoRedDatabaseProvider {
    pub datastore: Datastore,
    pub database_session: Session,
}

impl AvoRedDatabaseProvider {
    pub async fn register(config: Config) -> AvoRedDatabaseProvider {
        let datastore = Datastore::new("file://data/avored.db")
            .await
            .expect("there is issue with connecting with data/avored.db storage");

        let database_session = Session::for_db(
            config.database_namespace.clone(),
            config.database_name.clone(),
        );

        AvoRedDatabaseProvider {
            datastore,
            database_session,
        }
    }
}
