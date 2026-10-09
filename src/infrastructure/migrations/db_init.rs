use surrealdb_core::{dbs::Session, kvs::Datastore};

use crate::error::Result;
use crate::infrastructure::persistence::into_iter_objects;

const INITIAL_SCHEMA_MIGRATION: &str = "0001_initial_schema";
const WORKFLOW_SCHEMA_MIGRATION: &str = "0002_workflow_schema";

const REFRESH_DATABASE: bool = false;

/// Apply pending database migrations.
pub async fn run(datastore: &Datastore, session: &Session) -> Result<()> {
    datastore
        .execute(
            "DEFINE TABLE IF NOT EXISTS _migrations SCHEMALESS;",
            session,
            None,
        )
        .await?;

    if !migration_is_applied(datastore, session, INITIAL_SCHEMA_MIGRATION).await? {
        let schema = r#"
        DEFINE TABLE IF NOT EXISTS users SCHEMALESS;
            DEFINE FIELD name ON TABLE users TYPE string;
            DEFINE FIELD email ON TABLE users TYPE string;
            DEFINE FIELD password ON TABLE users TYPE string;
            DEFINE FIELD created_at ON TABLE users TYPE datetime;
            DEFINE FIELD created_by ON TABLE users TYPE string;
            DEFINE FIELD updated_at ON TABLE users TYPE datetime;
            DEFINE FIELD updated_by ON TABLE users TYPE string;
            DEFINE FIELD deleted_at ON TABLE users TYPE option<datetime>;
            DEFINE FIELD deleted_by ON TABLE users TYPE option<string>;

            DEFINE INDEX IF NOT EXISTS users_email_unique
                ON users FIELDS email UNIQUE;

        
        DEFINE TABLE IF NOT EXISTS entities SCHEMAFULL;
            DEFINE FIELD name ON TABLE entities TYPE string;
            DEFINE FIELD identifier ON TABLE entities TYPE string;
            DEFINE FIELD created_at ON TABLE entities TYPE datetime;
            DEFINE FIELD created_by ON TABLE entities TYPE string;
            DEFINE FIELD updated_at ON TABLE entities TYPE datetime;
            DEFINE FIELD updated_by ON TABLE entities TYPE string;
            DEFINE FIELD deleted_at ON TABLE entities TYPE option<datetime>;
            DEFINE FIELD deleted_by ON TABLE entities TYPE option<string>;

            DEFINE INDEX IF NOT EXISTS entities_identifier_unique
                ON entities FIELDS identifier UNIQUE;


        DEFINE TABLE IF NOT EXISTS attributes SCHEMAFULL;
            DEFINE FIELD entity_id ON TABLE attributes TYPE record<entities>;
            DEFINE FIELD name ON TABLE attributes TYPE string;
            DEFINE FIELD identifier ON TABLE attributes TYPE string;
            DEFINE FIELD field_type ON TABLE attributes TYPE string;
            DEFINE FIELD data_type ON TABLE attributes TYPE string;
            DEFINE FIELD created_at ON TABLE attributes TYPE datetime;
            DEFINE FIELD created_by ON TABLE attributes TYPE string;
            DEFINE FIELD updated_at ON TABLE attributes TYPE datetime;
            DEFINE FIELD updated_by ON TABLE attributes TYPE string;
            DEFINE FIELD deleted_at ON TABLE attributes TYPE option<datetime>;
            DEFINE FIELD deleted_by ON TABLE attributes TYPE option<string>;

            DEFINE INDEX IF NOT EXISTS attributes_identifier_unique
                ON attributes FIELDS identifier UNIQUE;

        DEFINE TABLE IF NOT EXISTS email_templates SCHEMAFULL;
            DEFINE FIELD name ON TABLE email_templates TYPE string;
            DEFINE FIELD subject ON TABLE email_templates TYPE string;
            DEFINE FIELD body_html ON TABLE email_templates TYPE string;
            DEFINE FIELD body_plain ON TABLE email_templates TYPE option<string>;
            DEFINE FIELD created_at ON TABLE email_templates TYPE datetime;
            DEFINE FIELD updated_at ON TABLE email_templates TYPE datetime;
            DEFINE FIELD deleted_at ON TABLE email_templates TYPE option<datetime>;
            DEFINE FIELD deleted_by ON TABLE email_templates TYPE option<string>;

            DEFINE INDEX IF NOT EXISTS email_templates_name_index
                ON email_templates FIELDS name UNIQUE;

        "#;

        datastore.execute(schema, session, None).await?;
        record_migration(
            datastore,
            session,
            INITIAL_SCHEMA_MIGRATION,
            "initial schema",
        )
        .await?;
    }

    if !migration_is_applied(datastore, session, WORKFLOW_SCHEMA_MIGRATION).await? {
        let workflow_schema = r#"
            DEFINE TABLE IF NOT EXISTS workflows SCHEMAFULL;
                DEFINE FIELD name ON TABLE workflows TYPE string;
                DEFINE FIELD trigger_entity ON TABLE workflows TYPE string;
                DEFINE FIELD trigger_event ON TABLE workflows TYPE string;
                DEFINE FIELD is_active ON TABLE workflows TYPE bool DEFAULT true;
                DEFINE FIELD actions ON TABLE workflows TYPE array<object> FLEXIBLE;
                DEFINE FIELD created_at ON TABLE workflows TYPE datetime DEFAULT time::now();
                DEFINE FIELD updated_at ON TABLE workflows TYPE datetime DEFAULT time::now();
                DEFINE FIELD deleted_at ON TABLE workflows TYPE option<datetime>;

            DEFINE TABLE IF NOT EXISTS workflow_executions SCHEMAFULL;
                DEFINE FIELD workflow_id ON TABLE workflow_executions TYPE string;
                DEFINE FIELD status ON TABLE workflow_executions TYPE string;
                DEFINE FIELD payload ON TABLE workflow_executions TYPE object FLEXIBLE;
                DEFINE FIELD error_message ON TABLE workflow_executions TYPE option<string>;
                DEFINE FIELD created_at ON TABLE workflow_executions TYPE datetime DEFAULT time::now();
                DEFINE FIELD updated_at ON TABLE workflow_executions TYPE datetime DEFAULT time::now();

            DEFINE INDEX IF NOT EXISTS workflows_trigger_active_index
                ON workflows FIELDS trigger_entity, trigger_event, is_active;
            DEFINE INDEX IF NOT EXISTS workflow_executions_workflow_created_index
                ON workflow_executions FIELDS workflow_id, created_at;
        "#;

        datastore.execute(workflow_schema, session, None).await?;
        record_migration(
            datastore,
            session,
            WORKFLOW_SCHEMA_MIGRATION,
            "workflow schema",
        )
        .await?;
    }

    if REFRESH_DATABASE {
        reset_local_database(datastore, session).await?;
    }

    Ok(())
}

async fn migration_is_applied(
    datastore: &Datastore,
    session: &Session,
    migration_id: &str,
) -> Result<bool> {
    if REFRESH_DATABASE {
        return Ok(false);
    }

    let responses = datastore
        .execute(
            &format!("SELECT * FROM _migrations:{};", migration_id),
            session,
            None,
        )
        .await?;

    Ok(into_iter_objects(responses)?.next().is_some())
}

async fn record_migration(
    datastore: &Datastore,
    session: &Session,
    migration_id: &str,
    name: &str,
) -> Result<()> {
    datastore
        .execute(
            &format!("CREATE _migrations:{migration_id} SET name = '{name}';"),
            session,
            None,
        )
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
             REMOVE TABLE IF EXISTS _migrations;",
            session,
            None,
        )
        .await?;

    Ok(())
}
