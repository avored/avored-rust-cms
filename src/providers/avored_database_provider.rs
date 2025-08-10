use crate::error::Result;
use crate::providers::avored_config_provider::AvoRedConfigProvider;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;



/// database dn type
pub type DB = (Datastore, Session);


/// avored database provider
pub struct AvoRedDatabaseProvider {

    /// database intance
    pub db: DB,
}

impl AvoRedDatabaseProvider {

    /// register avored database provider
    pub async fn register(config: AvoRedConfigProvider) -> Result<Self> {
        let folder_name = config.database_folder_name;
        let datastore = Datastore::new(&folder_name)
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

        Ok(Self { db })
    }
}
