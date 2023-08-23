use std::collections::BTreeMap;
use std::sync::Arc;
use axum::extract::State;
use axum::response::IntoResponse;
use surrealdb::dbs::Response;
use crate::avored_state::AvoRedState;
use crate::error::AvoRedError;
use crate::models::admin_user_model::AdminUser;
use crate::prelude::AvoRedResult;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn test_pp_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
) -> AvoRedResult<()> {

    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };
    let password_hash = String::from("test_password");

    let vars = BTreeMap::from([
        ("full_name".into(), "Test PP".into()),
        ("email".into(), "sdssdfdfsftest1@test.com".into()),
        ("password".into(), password_hash.as_str().into()),
        ("profile_image".into(), "".into()),
        ("is_super_admin".into(), false.into()),
        ("logged_in_user_name".into(), logged_in_user.full_name.as_str().into()),
    ]);

    let sql = "
        CREATE admin_users:test1_test_com CONTENT {
            full_name: $full_name,
            email: $email,
            password: $password,
            profile_image: $profile_image,
            is_super_admin: $is_super_admin,
            created_by: $logged_in_user_name,
            updated_by: $logged_in_user_name,
            created_at: time::now(),
            updated_at: time::now()
        };
    ";

    let responses = state
        .datastore
        .execute(sql, &state.database_session, Some(vars), false)
        .await?;

    let response = responses
        .into_iter()
        .next()
        .map(|response| response.result)
        .transpose()
        ;


    println!("Response: {:?}", response);

    Ok(())
}