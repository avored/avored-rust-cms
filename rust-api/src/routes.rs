use std::sync::Arc;
use axum::http::HeaderValue;
use axum::{Router, middleware};
use axum::routing::{get, post};
use axum::http::header::{CONTENT_TYPE, AUTHORIZATION};
use tower_http::cors::{CorsLayer};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::{Pool, ConnectionManager};

use crate::repositories::post_repository::PostRepository;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::middleware::require_authentication::require_authentication;
use crate::handlers::home_handler::home_handler;

use crate::handlers::admin_users_handler::admin_users_handler;
use crate::handlers::create_admin_user_handler::create_admin_user_handler;
use crate::handlers::login_admin_user_handler::login_admin_user_handler;

use crate::config::Config;

pub struct AppState {
    pub post_repository: PostRepository,
    pub admin_user_repository: AdminUserRepository,
    pub config: Config
}

pub fn app_routes() -> Router {
    let cors:CorsLayer = CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_headers([CONTENT_TYPE, AUTHORIZATION])
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS]);

    let db: Pool<ConnectionManager<PgConnection>> = establish_connection();
    // let connection = &mut db.get().unwrap();

    /************** REPOSITORIES  **************/
    let post_repository = PostRepository::new(db.clone());
    let admin_user_repository = AdminUserRepository::new(db.clone());

    let config: Config = Config::new();

    /************** APPLICATION STATES  **************/
    let app_state = Arc::new(AppState {
        post_repository,
        admin_user_repository,
        config
    });

    Router::new()
        .route("/", get(home_handler))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/api/admin_users", get(admin_users_handler))
        .route("/api/admin_users", post(create_admin_user_handler))
        .route("/api/auth/login", post(login_admin_user_handler))
        .with_state(app_state)
        .layer(cors)
}

pub fn establish_connection() ->  Pool<ConnectionManager<PgConnection>> {
    let config: Config = Config::new();
    let database_url: String = config.database_url;

    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool.")
}
