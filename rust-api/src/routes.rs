use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use axum::http::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::{Pool, ConnectionManager};

use crate::repositories::post_repository::PostRepository;
use crate::repositories::admin_user_repository::AdminUserRepository;

use crate::handlers::home_handler::home_handler;

use crate::handlers::admin_users_handler::admin_users_handler;
use crate::handlers::create_admin_user_handler::create_admin_user_handler;
use crate::handlers::login_admin_user_handler::login_admin_user_handler;

use crate::config::Config;

pub struct AppState {
    pub post_repository: PostRepository,
    pub admin_user_repository: AdminUserRepository
}

pub fn app_routes() -> Router {
    let cors:CorsLayer = CorsLayer::new().allow_origin(Any)
            .allow_headers([CONTENT_TYPE])
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST]);

    let db: Pool<ConnectionManager<PgConnection>> = establish_connection();
    // let connection = &mut db.get().unwrap();

    /************** REPOSITORIES  **************/
    let post_repository = PostRepository::new(db.clone());
    let admin_user_repository = AdminUserRepository::new(db.clone());


    /************** APPLICATION STATES  **************/
    let app_state = Arc::new(AppState {
        post_repository,
        admin_user_repository
    });

    Router::new()
        .route("/", get(home_handler))
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
