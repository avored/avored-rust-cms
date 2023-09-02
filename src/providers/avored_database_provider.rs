use surrealdb::{dbs::Session, kvs::Datastore};

use crate::providers::avored_config_provider::AvoRedConfigProvider;

pub struct AvoRedDatabaseProvider {
    pub datastore: Datastore,
    pub database_session: Session,
}

impl AvoRedDatabaseProvider {
    pub async fn register(config: AvoRedConfigProvider) -> AvoRedDatabaseProvider {
        let datastore = Datastore::new("file://data/avored.db")
            .await
            .expect("there is issue with connecting with data/avored.db storage");

        println!("ns:{} db: {}", config.database_namespace.clone(), config.database_name.clone());
        let database_session = Session::default()
            .with_ns(&config.database_namespace)
            .with_db(&config.database_name);
        // let database_session = Session::for_db(
        //     config.database_namespace.clone(),
        //     config.database_name.clone(),
        // );

        AvoRedDatabaseProvider {
            datastore,
            database_session,
        }
    }
}
