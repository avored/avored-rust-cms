use std::{collections::BTreeMap, sync::Arc};

use crate::error::Error;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use crate::{avored_state::AvoRedState, error::Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, response::IntoResponse, Json};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

pub async fn post_setup_avored_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<SetupAvoRedRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - post_setup_avored_handler", "HANDLER");

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    let sql = "

        REMOVE TABLE settings;
        DEFINE TABLE settings;

        DEFINE FIELD identifier ON TABLE settings TYPE string;
        DEFINE FIELD value      ON TABLE settings TYPE string;
        DEFINE FIELD created_at ON TABLE settings TYPE datetime;
        DEFINE FIELD updated_at ON TABLE settings TYPE datetime;
        DEFINE FIELD created_by ON TABLE settings TYPE string;
        DEFINE FIELD updated_by ON TABLE settings TYPE string;

        CREATE settings CONTENT {
            identifier: 'general_site_name',
            value: 'Avored rust cms',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };

        CREATE settings CONTENT {
            identifier: 'auth_cms_token',
            value: '',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };


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
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };

        REMOVE TABLE password_rest;
        DEFINE TABLE password_rest;

        DEFINE FIELD email ON TABLE password_rest TYPE string;
        DEFINE FIELD token ON TABLE password_rest TYPE string;
        DEFINE FIELD created_at ON TABLE password_rest TYPE datetime;


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
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };


        REMOVE TABLE admin_user_role;
        DEFINE TABLE admin_user_role;

        DEFINE FIELD created_by ON TABLE admin_user_role TYPE string;
        DEFINE FIELD updated_by ON TABLE admin_user_role TYPE string;
        DEFINE FIELD created_at ON TABLE admin_user_role TYPE datetime;
        DEFINE FIELD updated_at ON TABLE admin_user_role TYPE datetime;


        REMOVE TABLE components;
        DEFINE TABLE components;

        DEFINE FIELD name ON TABLE components TYPE string;
        DEFINE FIELD identifier ON TABLE components TYPE string;
        DEFINE FIELD created_by ON TABLE components TYPE string;
        DEFINE FIELD updated_by ON TABLE components TYPE string;
        DEFINE FIELD created_at ON TABLE components TYPE datetime;
        DEFINE FIELD updated_at ON TABLE components TYPE datetime;
        DEFINE INDEX components_identifier_index ON TABLE components COLUMNS identifier UNIQUE;


        REMOVE TABLE pages;
        DEFINE TABLE pages;

        DEFINE FIELD name ON TABLE pages TYPE string;
        DEFINE FIELD identifier ON TABLE pages TYPE string;
        DEFINE FIELD created_by ON TABLE pages TYPE string;
        DEFINE FIELD updated_by ON TABLE pages TYPE string;
        DEFINE FIELD created_at ON TABLE pages TYPE datetime;
        DEFINE FIELD updated_at ON TABLE pages TYPE datetime;
        DEFINE INDEX pages_identifier_index ON TABLE pages COLUMNS identifier UNIQUE;

        REMOVE TABLE assets;
        DEFINE TABLE assets;


        DEFINE TABLE fields;

        REMOVE TABLE collections;
        DEFINE TABLE collections;

        DEFINE FIELD name ON TABLE collections TYPE string;
        DEFINE FIELD identifier ON TABLE collections TYPE string;
        DEFINE FIELD created_by ON TABLE collections TYPE string;
        DEFINE FIELD updated_by ON TABLE collections TYPE string;
        DEFINE FIELD created_at ON TABLE collections TYPE datetime;
        DEFINE FIELD updated_at ON TABLE collections TYPE datetime;
        DEFINE INDEX collections_identifier_index ON TABLE collections COLUMNS identifier UNIQUE;
        DEFINE INDEX collections_identifier_index ON TABLE collections COLUMNS identifier UNIQUE;

        CREATE collections CONTENT {
            name: 'Pages',
            identifier: 'pages',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };

    ";

    let password = payload.password.as_bytes();
    let salt = SaltString::from_b64(&state.config.password_salt)?;

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
    println!();
    println!("Migrate fresh done!");

    let response = SetupViewModel { status: true };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
#[cfg_attr(test, derive(Deserialize, Eq, PartialEq, Copy, Clone, Default))]
pub struct SetupViewModel {
    pub status: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SetupAvoRedRequest {
    // #[validate(email(message = "The email field must be a valid email address."))]
    pub email: String,
    // #[validate(length(min = 1, message = "The password is a required field."))]
    pub password: String,
}

impl SetupAvoRedRequest {
    fn validate(&self) -> Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.email.is_empty() {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Email is a required field"),
            };

            errors.push(error_message);
        }

        if !EmailAddress::is_valid(&self.email) {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Invalid email address"),
            };

            errors.push(error_message);
        }

        if self.password.is_empty() {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: String::from("Password is a required field"),
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
