#![allow(unused)]

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use avored_better_query::AvoRedForm;
use axum::extract::{Multipart, State};
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use rand::distributions::Alphanumeric;
use rand::Rng;
use regex::Regex;
use std::collections::BTreeMap;
use std::path::Path;
use std::str::Bytes;
use std::sync::Arc;
use surrealdb::dbs::Response;
use urlencoding::decode_binary;
use validator::{HasLen, Validate, ValidationErrors, ValidationErrorsKind};

use crate::avored_state::AvoRedState;
use crate::models::ModelCount;
use crate::models::admin_user_model::{AdminUser, CreatableAdminUser};
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::ValidateRequest;
use crate::requests::store_role_request::StoreRoleRequest;

pub async fn store_role_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession,
    AvoRedForm(payload): AvoRedForm<StoreRoleRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let mut has_error = false;
    let validation_error_list = payload.validation_error(&mut session);
 

    let role_count = state
        .role_service
        .has_identifier_taken(
            &state.datastore,
            &state.database_session,
            payload.identifier.clone(),
        )
        .await
        .unwrap_or(ModelCount::new());

    if role_count.count >= 1 {
        has_error = true;
        session
            .insert(
                "validation_error_identifier",
                String::from("Identifier already taken. Please use another identifier"),
            )
            .expect("Could not store the validation errors into session.");
    }

    if has_error || validation_error_list.errors().length() > 0 {
        println!("{:?}", payload);
        return Err(Redirect::to("/admin/create-role").into_response());
    }

    // let creatable_admin_user = CreatableAdminUser {
    //     full_name: payload.full_name,
    //     email: payload.email,
    //     password: password_hash,
    //     profile_image,
    //     is_super_admin: payload.is_super_admin,
    //     logged_in_username: logged_in_user.email,
    // };

    // let created_admin_user = state
    //     .admin_user_repository
    //     .create_admin_user(&state.datastore, &state.database_session, creatable_admin_user)
    //     .await;

    Ok(Redirect::to("/admin/role").into_response())
}
