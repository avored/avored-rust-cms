
use std::sync::Arc;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::{AdminUser, ModelCount};
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn test_pp_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
) -> impl IntoResponse {

    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };
    // let admin_user_model = state.admin_user_repository.find_by_email(&state.datastore, &state.database_session, String::from("sdssdfdfsftes21@test.com")).await;
    //
    // let admin_model = match admin_user_model {
    //     Ok(admin_model) => admin_model,
    //     Err(err) => {
    //         AdminUser::empty_admin_user()
    //     }
    // };

    let admin_user_count = state
        .admin_user_repository
        .has_email_address_taken(
            &state.datastore,
            &state.database_session,
            logged_in_user.email
        )
        .await
        .unwrap_or(ModelCount::new());
    // let count = admin_user_count
    println!("{:?}", admin_user_count);
    // let password_hash = String::from("test_password");
    //
    // let vars = BTreeMap::from([
    //     ("full_name".into(), "Test PP".into()),
    //     ("email".into(), "sdssdfdfsftes21@test.com".into()),
    //     ("password".into(), password_hash.as_str().into()),
    //     ("profile_image".into(), "".into()),
    //     ("is_super_admin".into(), false.into()),
    //     ("logged_in_user_name".into(), logged_in_user.full_name.as_str().into()),
    // ]);
    //
    // // let sql = "
    // //     CREATE admin_users CONTENT {
    // //         full_name: $full_name,
    // //         email: $email,
    // //         password: $password,
    // //         profile_image: $profile_image,
    // //         is_super_admin: $is_super_admin,
    // //         created_by: $logged_in_user_name,
    // //         updated_by: $logged_in_user_name,
    // //         created_at: time::now(),
    // //         updated_at: time::now()
    // //     };
    // // ";
    // let sql = "SELECT count(email) from admin_users  where email=$email;";
    //
    // let responses = state
    //     .datastore
    //     .execute(sql, &state.database_session, Some(vars), false)
    //     .await?;
    //
    // let response = responses
    //     .into_iter()
    //     .next()
    //     .map(|response| response.result)
    //     .transpose()
    //     ;
    //
    // let response  = match response {
    //     Ok(res) => {
    //         println!("Ok error");
    //         res
    //     },
    //     Err(err) => {
    //         // if err.to_string()
    //         println!("transpose error {:?}", err.to_string());
    //         AvoRedError::Generic(String::from("record found"));
    //         // Err(err)
    //         Some(Value::None)
    //     }
    // };
    //
    // let res = match response {
    //     Some(Value::Array(array)) => {
    //         let res = array.into_iter().map(|v| match v {
    //             Value::Object(object) =>  {
    //                 println!("error object");
    //                 Ok(object)
    //             },
    //
    //             _ =>  {
    //                 println!("object matching error");
    //                 Err("Test")
    //             }
    //         });
    //         println!("OK");
    //
    //         Ok(res)
    //     },
    //     _ => {
    //         println!("find error");
    //         Err(AvoRedError::Generic(String::from("generic")))
    //     }
    // };
    // for object in res? {
    //     println!("Record {:?}", object?);
    // }
    //
    //
    // println!("Response: {:?}", res);

    Redirect::to("/admin").into_response()
}