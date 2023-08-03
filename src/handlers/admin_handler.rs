use axum::response::IntoResponse;
use axum::Json;
use surrealdb::dbs::Response;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Array;
use surrealdb::sql::Object;

use crate::error::Result;
use crate::models::admin_user_model::AdminUser;
use crate::models::W;

pub async fn admin_handler() -> impl IntoResponse {
    let datastore = Datastore::new("file://data/avored.db")
        .await
        .expect("there is issue with connecting with data/avored.db storage");

    let database_session = Session::for_db("public", "avored_cms");

    // let sql = "DELETE admin_users where name = 'Purvesh';";

    // let _responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };

    // let sql = "CREATE admin_users SET name = 'Purvesh';";

    // let _responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };
    let sql = "SELECT * FROM admin_users;";

    let responses = match datastore.execute(sql, &database_session, None, false).await {
        Ok(response) => response,
        Err(_) => {
            // todo improve this error
            let out: Vec<Response> = vec![];
            out
        }
    };

    let response = responses
        .into_iter()
        .next()
        .expect("error while retriving the first response");

    let result = response
        .result
        .expect("first result comes with error")
    ;

    let array: Array = W(result).try_into().expect("sdfds");
    let objects: Result<Vec<Object>> = array.into_iter().map(|value| W(value).try_into()).collect();
    let objects = match objects {
        Ok(obj) => obj,
        Err(_) => panic!("no data"),
    };

    let test: Result<Vec<AdminUser>> = objects.into_iter().map(|o| o.try_into()).collect();

    let admin_users = match test {
        Ok(data) => data,
        Err(_) => panic!("some errror"),
    };
    // let test: Result<AdminUser, Error> = test.try_into();

    // println!("Responses: {:?}", test);

    Json(admin_users).into_response()
}
