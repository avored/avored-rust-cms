use crate::error::Result;
use crate::providers::avored_config_provider::AvoRedConfigProvider;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;

pub type DB = (Datastore, Session);
pub struct AvoRedDatabaseProvider {
    pub db: DB,
}

impl AvoRedDatabaseProvider {
    pub async fn register(config: AvoRedConfigProvider) -> Result<AvoRedDatabaseProvider> {
        let datastore = Datastore::new("file://data/avored.db")
            .await
            .expect("there is issue with connecting with data/avored.db storage");

        println!(
            "ns:{} db: {}",
            config.database_namespace.clone(),
            config.database_name.clone()
        );
        let database_session = Session::default()
            .with_ns(&config.database_namespace)
            .with_db(&config.database_name);

        let db = (datastore, database_session);

        Ok(AvoRedDatabaseProvider { db })
    }
}
