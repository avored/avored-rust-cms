use crate::error::Result;
use surrealdb_core::dbs::Session;
use surrealdb_core::kvs::Datastore;

/// database dn type
pub type DB = (Datastore, Session);

/// avored database provider
pub struct AvoRedDatabaseProvider {
    pub db: DB,
}

impl AvoRedDatabaseProvider {
    /// register avored database provider
    pub async fn register(
        database_folder_name: &str,
        database_namespace: &str,
        database_name: &str,
    ) -> Result<Self> {
        let folder_name = database_folder_name;

        //  let db = Surreal::new::<RocksDb>("path/to/database-folder").await?;
        // // let test = surrealdb::engine::local::Folder::new(folder_name);

        let datastore = Datastore::new(&folder_name)
            .await
            .expect("there is issue with connecting with data/avored.db storage");

        // 1. Ensure Namespace exists (Root/Owner level session)
        let root_session = Session::owner();
        let define_ns_sql = format!("DEFINE NAMESPACE IF NOT EXISTS `{}`;", database_namespace);
        datastore
            .execute(&define_ns_sql, &root_session, None)
            .await?;
        // 2. Ensure Database exists within Namespace
        let ns_session = Session::owner().with_ns(database_namespace);
        let define_db_sql = format!("DEFINE DATABASE IF NOT EXISTS `{}`;", database_name);
        datastore.execute(&define_db_sql, &ns_session, None).await?;

        let database_session = Session::owner()
            .with_ns(&database_namespace)
            .with_db(&database_name);

        // 3. Ensure essential tables exist
        let define_tables_sql = "
            DEFINE TABLE IF NOT EXISTS users SCHEMALESS;
            DEFINE TABLE IF NOT EXISTS entities SCHEMALESS;
        ";
        datastore.execute(define_tables_sql, &database_session, None).await?;

        let db = (datastore, database_session);

        Ok(Self { db })
    }
}
