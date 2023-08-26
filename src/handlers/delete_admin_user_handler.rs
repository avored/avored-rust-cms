use std::collections::BTreeMap;
use std::sync::Arc;
use std::path::Path as stdPath;
use axum::extract::{State, Path, Multipart};
use axum::response::{Html, IntoResponse, Redirect};
use rand::Rng;
use rand::distributions::Alphanumeric;
use surrealdb::dbs::Response;
use urlencoding::decode_binary;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::update_admin_user_request::UpdateAdminUserRequest;

pub async fn delete_admin_user_handler(
    state: State<Arc<AvoRedState>>,
    Path(admin_user_id): Path<String>,
    mut session: AvoRedSession,
) -> impl IntoResponse  {

    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let sql = "DELETE type::thing($table, $id)";

    let vars = BTreeMap::from([
        ("id".into(), admin_user_id.into()),
        ("table".into(), "admin_users".into()),
    ]);

    let responses = match state.datastore.execute(sql, &state.database_session, Some(vars), false).await {
        Ok(response) => response,
        Err(_) => {
            let out: Vec<Response> = vec![];
            out
        }
    };
    
    session
        .insert(
            "delete_success_message",
            String::from("Admin user deleted successfully"),
        )
        .expect("Could not store the validation errors into session.");


    // println!("{:?}", responses);

    Html("").into_response()
}
