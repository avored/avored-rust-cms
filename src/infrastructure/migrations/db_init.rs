use std::collections::BTreeMap;
use surrealdb::types::Value;
use surrealdb_core::{dbs::Session, kvs::Datastore};

use crate::core::domain::extensions::RecordIdExtension;
use crate::error::Result;

pub const SCHEMA_MIGRATION_V1: &str = "0001_schema_v1";

const MERGED_SCHEMA: &str = r#"
DEFINE TABLE IF NOT EXISTS users SCHEMALESS;
    DEFINE FIELD IF NOT EXISTS name ON TABLE users TYPE string;
    DEFINE FIELD IF NOT EXISTS email ON TABLE users TYPE string;
    DEFINE FIELD IF NOT EXISTS password ON TABLE users TYPE string;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE users TYPE datetime;
    DEFINE FIELD IF NOT EXISTS created_by ON TABLE users TYPE string;
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE users TYPE datetime;
    DEFINE FIELD IF NOT EXISTS updated_by ON TABLE users TYPE string;
    DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE users TYPE option<datetime>;
    DEFINE FIELD IF NOT EXISTS deleted_by ON TABLE users TYPE option<string>;

    DEFINE INDEX IF NOT EXISTS users_email_unique
        ON users FIELDS email UNIQUE;

DEFINE TABLE IF NOT EXISTS entities SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS name ON TABLE entities TYPE string;
    DEFINE FIELD IF NOT EXISTS identifier ON TABLE entities TYPE string;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE entities TYPE datetime;
    DEFINE FIELD IF NOT EXISTS created_by ON TABLE entities TYPE string;
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE entities TYPE datetime;
    DEFINE FIELD IF NOT EXISTS updated_by ON TABLE entities TYPE string;
    DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE entities TYPE option<datetime>;
    DEFINE FIELD IF NOT EXISTS deleted_by ON TABLE entities TYPE option<string>;

    DEFINE INDEX IF NOT EXISTS entities_identifier_unique
        ON entities FIELDS identifier UNIQUE;

DEFINE TABLE IF NOT EXISTS attributes SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS entity_id ON TABLE attributes TYPE record<entities>;
    DEFINE FIELD IF NOT EXISTS name ON TABLE attributes TYPE string;
    DEFINE FIELD IF NOT EXISTS identifier ON TABLE attributes TYPE string;
    DEFINE FIELD IF NOT EXISTS field_type ON TABLE attributes TYPE string;
    DEFINE FIELD IF NOT EXISTS data_type ON TABLE attributes TYPE string;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE attributes TYPE datetime;
    DEFINE FIELD IF NOT EXISTS created_by ON TABLE attributes TYPE string;
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE attributes TYPE datetime;
    DEFINE FIELD IF NOT EXISTS updated_by ON TABLE attributes TYPE string;
    DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE attributes TYPE option<datetime>;
    DEFINE FIELD IF NOT EXISTS deleted_by ON TABLE attributes TYPE option<string>;

    DEFINE INDEX IF NOT EXISTS attributes_identifier_unique
        ON attributes FIELDS identifier UNIQUE;

DEFINE TABLE IF NOT EXISTS email_templates SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS name ON TABLE email_templates TYPE string;
    DEFINE FIELD IF NOT EXISTS subject ON TABLE email_templates TYPE string;
    DEFINE FIELD IF NOT EXISTS body_html ON TABLE email_templates TYPE string;
    DEFINE FIELD IF NOT EXISTS body_plain ON TABLE email_templates TYPE option<string>;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE email_templates TYPE datetime;
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE email_templates TYPE datetime;
    DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE email_templates TYPE option<datetime>;
    DEFINE FIELD IF NOT EXISTS deleted_by ON TABLE email_templates TYPE option<string>;

    DEFINE INDEX IF NOT EXISTS email_templates_name_index
        ON email_templates FIELDS name UNIQUE;

DEFINE TABLE IF NOT EXISTS workflows SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS name ON TABLE workflows TYPE string;
    DEFINE FIELD IF NOT EXISTS trigger_entity ON TABLE workflows TYPE string;
    DEFINE FIELD IF NOT EXISTS trigger_event ON TABLE workflows TYPE string;
    DEFINE FIELD IF NOT EXISTS is_active ON TABLE workflows TYPE bool DEFAULT true;
    DEFINE FIELD IF NOT EXISTS actions ON TABLE workflows TYPE array<object> FLEXIBLE;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE workflows TYPE datetime DEFAULT time::now();
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE workflows TYPE datetime DEFAULT time::now();
    DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE workflows TYPE option<datetime>;

DEFINE TABLE IF NOT EXISTS workflow_executions SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS workflow_id ON TABLE workflow_executions TYPE string;
    DEFINE FIELD IF NOT EXISTS status ON TABLE workflow_executions TYPE string;
    DEFINE FIELD IF NOT EXISTS payload ON TABLE workflow_executions TYPE object FLEXIBLE;
    DEFINE FIELD IF NOT EXISTS error_message ON TABLE workflow_executions TYPE option<string>;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE workflow_executions TYPE datetime DEFAULT time::now();
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE workflow_executions TYPE datetime DEFAULT time::now();

    DEFINE INDEX IF NOT EXISTS workflows_trigger_active_index
        ON workflows FIELDS trigger_entity, trigger_event, is_active;
    DEFINE INDEX IF NOT EXISTS workflow_executions_workflow_created_index
        ON workflow_executions FIELDS workflow_id, created_at;

DEFINE TABLE IF NOT EXISTS api_keys SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS name        ON TABLE api_keys TYPE string;
    DEFINE FIELD IF NOT EXISTS key_prefix  ON TABLE api_keys TYPE string;
    DEFINE FIELD IF NOT EXISTS key_hash    ON TABLE api_keys TYPE string;
    DEFINE FIELD IF NOT EXISTS permissions ON TABLE api_keys TYPE array<object> FLEXIBLE;
    DEFINE FIELD IF NOT EXISTS created_at  ON TABLE api_keys TYPE datetime;
    DEFINE FIELD IF NOT EXISTS created_by  ON TABLE api_keys TYPE string;
    DEFINE FIELD IF NOT EXISTS updated_at  ON TABLE api_keys TYPE datetime;
    DEFINE FIELD IF NOT EXISTS updated_by  ON TABLE api_keys TYPE string;

    DEFINE INDEX IF NOT EXISTS api_keys_hash_unique
        ON api_keys FIELDS key_hash UNIQUE;

DEFINE TABLE IF NOT EXISTS settings SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS identifier ON TABLE settings TYPE string;
    DEFINE FIELD IF NOT EXISTS value ON TABLE settings TYPE string;
    DEFINE FIELD IF NOT EXISTS setting_group ON TABLE settings TYPE string;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE settings TYPE datetime;
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE settings TYPE datetime;

    DEFINE INDEX IF NOT EXISTS settings_identifier_unique
        ON settings FIELDS identifier UNIQUE;
"#;

/// Apply pending database migrations.
pub async fn run(datastore: &Datastore, session: &Session) -> Result<()> {
    datastore
        .execute(
            "DEFINE TABLE IF NOT EXISTS _migrations SCHEMALESS;
             DEFINE FIELD IF NOT EXISTS name ON TABLE _migrations TYPE string;
             DEFINE FIELD IF NOT EXISTS applied_at ON TABLE _migrations TYPE datetime DEFAULT time::now();",
            session,
            None,
        )
        .await?;

    tracing::info!("applying merged database schema");
    datastore.execute(MERGED_SCHEMA, session, None).await?;

    record_migration(
        datastore,
        session,
        SCHEMA_MIGRATION_V1,
        "merged schema v1",
    )
    .await?;
    tracing::info!(migration_id = SCHEMA_MIGRATION_V1, "merged database schema applied successfully");

    Ok(())
}

pub async fn record_migration(
    datastore: &Datastore,
    session: &Session,
    migration_id: &str,
    name: &str,
) -> Result<()> {
    let sql = "CREATE $id SET name = $name, applied_at = time::now();";
    let params = BTreeMap::from([
        (
            "id".to_string(),
            Value::RecordId(migration_id.to_record_id("_migrations")),
        ),
        ("name".to_string(), Value::String(name.to_string())),
    ]);

    datastore
        .execute(sql, session, Some(params.into()))
        .await?;
    Ok(())
}

/// Reset application tables in a local database.
///
/// This is intentionally separate from [`run`] so normal application startup
/// never deletes data. Call it explicitly from local reset tooling only.
pub async fn reset_local_database(datastore: &Datastore, session: &Session) -> Result<()> {
    datastore
        .execute(
            "REMOVE TABLE IF EXISTS users;
             REMOVE TABLE IF EXISTS entities;
             REMOVE TABLE IF EXISTS attributes;
             REMOVE TABLE IF EXISTS email_templates;
             REMOVE TABLE IF EXISTS workflows;
             REMOVE TABLE IF EXISTS workflow_executions;
             REMOVE TABLE IF EXISTS api_keys;
             REMOVE TABLE IF EXISTS settings;
             REMOVE TABLE IF EXISTS _migrations;",
            session,
            None,
        )
        .await?;

    Ok(())
}
