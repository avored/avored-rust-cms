use std::{collections::BTreeMap, sync::Arc};

use crate::{
    avored_state::AvoRedState,
    error::Result,
};
use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHasher,
};
use axum::{extract::State, Json, response::IntoResponse};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};


pub async fn post_setup_avored_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<SetupAvoRedRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - post_setup_avored_handler", "HANDLER");

    let error_messages = payload.validate()?;

    if error_messages.len() > 0 {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }


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
            created_by: $full_name,
            updated_by: $full_name,
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



        REMOVE TABLE fields;
        DEFINE TABLE fields;

        DEFINE FIELD name ON TABLE fields TYPE string;
        DEFINE FIELD identifier ON TABLE fields TYPE string;
        DEFINE FIELD field_type ON TABLE fields TYPE string;
        DEFINE FIELD created_by ON TABLE fields TYPE string;
        DEFINE FIELD updated_by ON TABLE fields TYPE string;
        DEFINE FIELD created_at ON TABLE fields TYPE datetime;
        DEFINE FIELD updated_at ON TABLE fields TYPE datetime;
        DEFINE INDEX fields_identifier_index ON TABLE fields COLUMNS identifier UNIQUE;







        REMOVE TABLE pages;
        DEFINE TABLE pages;

        DEFINE FIELD name ON TABLE pages TYPE string;
        DEFINE FIELD identifier ON TABLE pages TYPE string;
        DEFINE FIELD components_content ON TABLE pages TYPE array;
        DEFINE FIELD created_by ON TABLE pages TYPE string;
        DEFINE FIELD updated_by ON TABLE pages TYPE string;
        DEFINE FIELD created_at ON TABLE pages TYPE datetime;
        DEFINE FIELD updated_at ON TABLE pages TYPE datetime;
        DEFINE INDEX pages_identifier_index ON TABLE pages COLUMNS identifier UNIQUE;

        CREATE pages CONTENT {
            name: 'Home Page',
            identifier: 'home-page',
            components_content: [{id: 'test id', name: 'test name', identifier: 'test identifier', component_fields_content: [{id: 'test id', name: 'test name', identifier: 'test identifier', field_type: 'text', field_content: 'test field content 1'}] }],
            created_by: $full_name,
            updated_by: $full_name,
            created_at: time::now(),
            updated_at: time::now()
        };


        REMOVE TABLE assets;
        DEFINE TABLE assets;

        DEFINE FIELD file_name      ON TABLE assets TYPE string;
        DEFINE FIELD file_path      ON TABLE assets TYPE string;
        DEFINE FIELD file_size      ON TABLE assets TYPE int;
        DEFINE FIELD file_type      ON TABLE assets TYPE string;
        DEFINE FIELD information    ON TABLE assets TYPE string;
        DEFINE FIELD created_by ON TABLE assets TYPE string;
        DEFINE FIELD updated_by ON TABLE assets TYPE string;
        DEFINE FIELD created_at ON TABLE assets TYPE datetime;
        DEFINE FIELD updated_at ON TABLE assets TYPE datetime;
    ";

    let password = payload.password.as_str().as_bytes();
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

    let response = SetupViewModel {
        status: true
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
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

        if self.email.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Email is a required field")
            };

            errors.push(error_message);
        }

        if ! EmailAddress::is_valid(&self.email) {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Invalid email address")
            };

            errors.push(error_message);
        }

        if self.password.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: String::from("Password is a required field")
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}