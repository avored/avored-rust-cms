use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::dbs::Response;

use crate::avored_state::AvoRedState;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn delete_role_handler(
    state: State<Arc<AvoRedState>>,
    Path(role_id): Path<String>,
    mut session: AvoRedSession,
) -> impl IntoResponse {
    let sql = "DELETE type::thing($table, $id)";

    let vars = BTreeMap::from([
        ("id".into(), role_id.into()),
        ("table".into(), "roles".into()),
    ]);

    let _responses = match state
        .datastore
        .execute(sql, &state.database_session, Some(vars))
        .await
    {
        Ok(response) => response,
        Err(_) => {
            let out: Vec<Response> = vec![];
            out
        }
    };

    session
        .insert("success_message", String::from("Role deleted successfully"))
        .expect("Could not store the validation errors into session.");

    Redirect::to("/admin/role").into_response()
    // Html("").into_response()
}
