use std::collections::BTreeMap;
use std::sync::Arc;
use axum::Form;
use axum::extract::{State, Path};
use axum::response::{IntoResponse, Redirect};
use surrealdb::dbs::Response;
use validator::{ValidationErrors, ValidationErrorsKind, HasLen, Validate};

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::update_admin_user_request::UpdateAdminUserRequest;

pub async fn update_admin_user_handler(
    state: State<Arc<AvoRedState>>,
    Path(admin_user_id): Path<String>,
    mut session: AvoRedSession,
    Form(payload): Form<UpdateAdminUserRequest>,
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

                    if !message.is_empty() {
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
        let redirect_url = format!("{}{}", String::from("/admin/edit-admin-user/"), admin_user_id);
        return Err(Redirect::to(&redirect_url).into_response());
    }

    let sql = "
    
    UPDATE type::thing($table, $id) MERGE {
        full_name: $full_name,
        updated_by: $logged_in_user_name,
        updated_at: time::now()
    };";

    let vars = BTreeMap::from([
        ("full_name".into(), payload.full_name.into()),
        ("logged_in_user_name".into(), logged_in_user.full_name.as_str().into()),
        ("id".into(), admin_user_id.into()),
        ("table".into(), "admin_users".into()),
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
