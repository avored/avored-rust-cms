use surrealdb_core::{dbs::Session, kvs::Datastore};

use crate::error::Result;
use crate::infrastructure::persistence::into_iter_objects;

const INITIAL_SCHEMA_MIGRATION: &str = "0001_initial_schema";

/// Apply pending database migrations.
pub async fn run(datastore: &Datastore, session: &Session) -> Result<()> {
    datastore
        .execute(
            "DEFINE TABLE IF NOT EXISTS _migrations SCHEMALESS;",
            session,
            None,
        )
        .await?;

    if migration_is_applied(datastore, session).await? {
        return Ok(());
    }

    let schema = r#"
        DEFINE TABLE IF NOT EXISTS users SCHEMALESS;
        DEFINE TABLE IF NOT EXISTS entities SCHEMALESS;
        DEFINE INDEX IF NOT EXISTS entities_identifier_unique
            ON entities FIELDS identifier UNIQUE;
    "#;

    datastore.execute(schema, session, None).await?;
    datastore
        .execute(
            &format!(
                "CREATE _migrations:{} SET name = 'initial schema';",
                INITIAL_SCHEMA_MIGRATION
            ),
            session,
            None,
        )
        .await?;

    Ok(())
}

async fn migration_is_applied(datastore: &Datastore, session: &Session) -> Result<bool> {
    let responses = datastore
        .execute(
            &format!(
                "SELECT * FROM _migrations:{};",
                INITIAL_SCHEMA_MIGRATION
            ),
            session,
            None,
        )
        .await?;

    Ok(into_iter_objects(responses)?.next().is_some())
}