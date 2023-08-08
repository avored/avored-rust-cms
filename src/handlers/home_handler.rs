use std::{sync::Arc, collections::BTreeMap};

use argon2::{password_hash::{SaltString, rand_core}, Argon2, PasswordHasher};
use axum::{response::{IntoResponse, Redirect}, extract::State};
use surrealdb::dbs::Response;
use rand_core::OsRng;

use crate::avored_state::AvoRedState;

pub async fn home_handler(state: State<Arc<AvoRedState>>,) -> impl IntoResponse {

    let password = "admin123";
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let vars = BTreeMap::from([
        ("full_name".into(), "Admin".into()),
        ("email".into(), "admin@admin.com".into()),
        ("password".into(), password_hash.as_str().into()),
    ]);
    // let datastore = &state.datastore;
    // let database_session = &state.database_session;

    let sql = "
        REMOVE TABLE admin_users;
        DEFINE TABLE admin_users;

        DEFINE FIELD full_name ON TABLE admin_users TYPE string;
        DEFINE FIELD email ON TABLE admin_users TYPE string;
        DEFINE FIELD password ON TABLE admin_users TYPE string;
        DEFINE FIELD created_by ON TABLE admin_users TYPE string;
        DEFINE FIELD updated_by ON TABLE admin_users TYPE string;
        DEFINE FIELD created_at ON TABLE admin_users TYPE datetime;
        DEFINE FIELD updated_at ON TABLE admin_users TYPE datetime;
        DEFINE INDEX admin_users_email_index ON TABLE admin_users COLUMNS email UNIQUE;

        CREATE admin_users CONTENT {
            full_name: $full_name,
            email: $email,
            password: $password,
            created_by: $full_name,
            updated_by: $full_name,
            created_at: time::now(),
            updated_at: time::now()
        };
    ";

    let _responses = match state.datastore.execute(sql, &state.database_session, Some(vars), false).await {
        Ok(response) => response,
        Err(_) => {
            // todo improve this error
            let out: Vec<Response> = vec![];
            out
        }
    };

    // let sql = "
        
    // ";

    // let _responses = match state.datastore.execute(sql, &state.database_session, Some(vars), false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };


    Redirect::to("/admin/login").into_response()
}
