use std::sync::Arc;

use crate::routes::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use serde_json::json;
// use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub async fn require_authentication<T>(
    app_state: State<Arc<AppState>>,
    _headers: HeaderMap,
    mut _request: Request<T>,
    _next: Next<T>,
) -> Result<Response, impl IntoResponse> {
    let data = json!({});
    let html = app_state.handlebars.render("401", &data).unwrap();

    // println!("I am required authentication {:?}", request);

    Err(Html(html).into_response())

    //  Err(AppError::new(
    //     StatusCode::UNAUTHORIZED,
    //     "You are not authorized for this",
    // ))

    // *request.uri_mut() = "/admin/login".parse().unwrap();

    // Ok(next.run(request).await)

    // Ok(Redirect::to("/admin/login"))

    // let full_token = headers.get("Authorization").unwrap();

    // let authorized_token = full_token.to_str().unwrap().replace("Bearer ", "");

    // // let token = authorized_token[6..];

    // let decoded = decode::<Claims>(
    //     &authorized_token,
    //     &DecodingKey::from_secret(app_state.config.jwt_secret.as_ref()),
    //     &Validation::new(Algorithm::HS256),
    // );

    // if decoded.is_ok() {
    //     let token_data = decoded.unwrap();

    //     let user = AdminUser {
    //         id: token_data.claims.sub,
    //         name: token_data.claims.name,
    //         email: token_data.claims.email,
    //         // password: token_data.claims.password,
    //         created_at: token_data.claims.created_at,
    //         updated_at: token_data.claims.updated_at,
    //         created_by: token_data.claims.created_by,
    //         updated_by: token_data.claims.updated_by,
    //     };

    //     request.extensions_mut().insert(user);

    //     *request.uri_mut() = "/admin/login".parse().unwrap();

    //     Ok(next.run(request).await)
    // } else {
    //     Err(AppError::new(
    //         StatusCode::UNAUTHORIZED,
    //         "You are not authorized for this",
    //     ))
    // }
}
