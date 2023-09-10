use std::{collections::BTreeMap, sync::Arc};

use crate::{
    api::setup::requests::setup_avored_request::SetupAvoRedRequest, avored_state::AvoRedState,
    error::Result,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use avored_better_query::AvoRedForm;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use serde::Serialize;

pub async fn post_setup_avored_handler(
    state: State<Arc<AvoRedState>>,
    AvoRedForm(payload): AvoRedForm<SetupAvoRedRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - post_setup_avored_handler", "HANDLER");

    let sql = "
        REMOVE TABLE admin_users;
        DEFINE TABLE admin_users;

        DEFINE FIELD full_name ON TABLE admin_users TYPE string;
        DEFINE FIELD email ON TABLE admin_users TYPE string;
        DEFINE FIELD password ON TABLE admin_users TYPE string;
        DEFINE FIELD profile_image ON TABLE admin_users TYPE string;
        DEFINE FIELD is_super_admin ON TABLE admin_users TYPE bool;
        DEFINE FIELD created_by ON TABLE admin_users TYPE string;
        DEFINE FIELD updated_by ON TABLE admin_users TYPE string;
        DEFINE FIELD created_at ON TABLE admin_users TYPE datetime;
        DEFINE FIELD updated_at ON TABLE admin_users TYPE datetime;
        DEFINE INDEX admin_users_email_index ON TABLE admin_users COLUMNS email UNIQUE;

        CREATE admin_users CONTENT {
            full_name: $full_name,
            email: $email,
            password: $password,
            profile_image: $profile_image,
            is_super_admin: $is_super_admin,
            created_by: $full_name,
            updated_by: $full_name,
            created_at: time::now(),
            updated_at: time::now()
        };



        REMOVE TABLE roles;
        DEFINE TABLE roles;

        DEFINE FIELD name ON TABLE roles TYPE string;
        DEFINE FIELD identifier ON TABLE roles TYPE string;
        DEFINE FIELD created_by ON TABLE roles TYPE string;
        DEFINE FIELD updated_by ON TABLE roles TYPE string;
        DEFINE FIELD created_at ON TABLE roles TYPE datetime;
        DEFINE FIELD updated_at ON TABLE roles TYPE datetime;
        DEFINE INDEX roles_identifier_index ON TABLE roles COLUMNS identifier UNIQUE;

        CREATE roles CONTENT {
            name: 'Administrator',
            identifier: 'administrator',
            created_by: $full_name,
            updated_by: $full_name,
            created_at: time::now(),
            updated_at: time::now()
        };
    ";

    let password = payload.password.as_str();
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let vars = BTreeMap::from([
        ("full_name".into(), "Admin".into()),
        ("email".into(), payload.email.into()),
        ("password".into(), password_hash.as_str().into()),
        ("profile_image".into(), "".into()),
        ("is_super_admin".into(), true.into()),
    ]);

    let (ds, ses) = &state.db;

    let responses = ds.execute(sql, ses, Some(vars)).await?;

    println!("{responses:?}");
    println!("");
    println!("Migrate fresh done!");

    Ok(Redirect::to("/admin/login"))
}

#[derive(Serialize)]
pub struct SetupViewModel {}
