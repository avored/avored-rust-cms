use std::collections::BTreeMap;
use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use axum::Form;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use serde_derive::Serialize;
use surrealdb::dbs::Response;
use validator::{ValidationErrors, ValidationErrorsKind, HasLen, Validate};

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::store_admin_user_request::StoreAdminUserRequest;

pub async fn store_admin_user_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession,
    Form(payload): Form<StoreAdminUserRequest>,
) -> Result<impl IntoResponse, impl IntoResponse>  {

    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let validation_error_list = match payload.validate() {
        Ok(_) => ValidationErrors::new(),
        Err(errors) => errors,
    };

    for (field_name, error) in validation_error_list.errors() {
        // let test = validation_error_list.errors();
        // let test= error::add("sdfs");
        match &error {
            ValidationErrorsKind::Field(field_errors) => {
                for field_error in field_errors {
                    let message = match &field_error.message {
                        Some(message) => message,
                        None => continue,
                    };
                    println!("{:?}", message.is_empty());

                    if !message.is_empty() {
                        // let key = field_name.clone();
                        let validation_key = format!("validation_error_{}", field_name);
                        session
                            .insert(&validation_key, message)
                            .expect("Could not store the validation errors into session.");
                    }
                }
            }
            ValidationErrorsKind::Struct(_) => continue,
            ValidationErrorsKind::List(_) => continue,
        }
    }
    if validation_error_list.errors().length() > 0 {
        return Err(Redirect::to("/admin/create-admin-user").into_response());
    }

    let sql = "
    
    CREATE admin_users CONTENT {
        full_name: $full_name,
        email: $email,
        password: $password,
        created_by: $logged_in_user_name,
        updated_by: $logged_in_user_name,
        created_at: time::now(),
        updated_at: time::now()
    };
";


let password = payload.password;
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let vars = BTreeMap::from([
        ("full_name".into(), payload.full_name.into()),
        ("email".into(), payload.email.into()),
        ("password".into(), password_hash.as_str().into()),
        ("logged_in_user_name".into(), logged_in_user.full_name.as_str().into()),
    ]);

    let responses = match state.datastore.execute(sql, &state.database_session, Some(vars), false).await {
        Ok(response) => response,
        Err(_) => {
            // todo improve this error
            let out: Vec<Response> = vec![];
            out
        }
    };

    println!("{:?}", responses);
    //@try to find the error here?
    
    Ok(Redirect::to("/admin/admin-user").into_response())

    // Json(admin_users).into_response()
}
