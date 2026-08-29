use crate::error::Result;
use surrealdb_core::kvs::Datastore;
use surrealdb_core::dbs::Session;




/// database dn type
pub type DB = (Datastore, Session);


/// avored database provider
pub struct AvoRedDatabaseProvider {

    pub db: DB,
}

impl AvoRedDatabaseProvider {

    /// register avored database provider
    pub async fn register(database_folder_name: &str, database_namespace: &str, database_name: &str) -> Result<Self> {
        let folder_name = database_folder_name;

        //  let db = Surreal::new::<RocksDb>("path/to/database-folder").await?;
        // // let test = surrealdb::engine::local::Folder::new(folder_name);

        let datastore = Datastore::new(&folder_name)
            .await
            .expect("there is issue with connecting with data/avored.db storage");

        println!(
            "ns:{} db: {}",
            database_namespace.clone(),
            database_name.clone()
        );
        let database_session = Session::owner()
            .with_ns(&database_namespace)
            .with_db(&database_name);

        let db = (datastore, database_session);

        Ok(Self { db })
    }
}
