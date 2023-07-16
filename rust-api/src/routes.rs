use crate::handlers::delete_role_handler::delete_role_handler;
use crate::handlers::get_admin_handler::get_admin_handler;
use crate::handlers::get_admin_login_handler::get_admin_login_handler;
use crate::handlers::home_handler::home_handler;
use crate::handlers::post_admin_login_handler::post_admin_login_handler;
use crate::handlers::put_role_handler::put_role_handler;
use crate::handlers::roles_handler::roles_handler;
use crate::middleware::require_authentication::require_authentication;
use crate::repositories::admin_user_repository::{AdminUser, AdminUserRepository};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderValue;
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};
use axum_sessions::async_session::MemoryStore;
use axum_sessions::SessionLayer;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use r_i18n::{I18n, I18nConfig};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::handlers::admin_users_handler::admin_users_handler;
use crate::handlers::create_admin_user_handler::create_admin_user_handler;
use crate::handlers::create_role_handler::create_role_handler;
use crate::handlers::delete_admin_user_handler::delete_admin_user_handler;
use crate::handlers::get_admin_user_handler::get_admin_user_handler;
use crate::handlers::get_role_handler::get_role_handler;
use crate::handlers::put_admin_user_handler::put_admin_user_handler;
use crate::handlers::test_pp_handler::test_pp_handler;
use crate::repositories::role_repository::RoleRepository;
use sea_orm::Database;

use crate::config::Config;

pub struct AppState {
    pub admin_user_repository: AdminUserRepository,
    pub role_repository: RoleRepository,
    pub config: Config,
    pub handlebars: Handlebars<'static>,
    pub current_user: Option<AdminUser>,
}

pub async fn app_routes() -> Router {
    let config: Config = Config::new();

    let cors_layer: CorsLayer = CorsLayer::new()
        .allow_origin(config.backend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ]);
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".hbs", "./views")
        .expect("handlebars cant register the views path");

    handlebars.register_helper("translate_key", Box::new(translate_key));
    handlebars.register_helper("get_validation_message", Box::new(get_validation_message));

    /************** REPOSITORIES  **************/
    let admin_user_repository = AdminUserRepository::new();
    let role_repository = RoleRepository::new();

    /************** APPLICATION STATES  **************/
    let app_state = Arc::new(AppState {
        admin_user_repository,
        role_repository,
        config,
        current_user: None,
        handlebars,
    });

    let public_static_service = ServeDir::new("public");

    let store = MemoryStore::new();
    // let secret = "secretKEy".as_str().unwrap();
    // let session_secret = ;
    // let secret = session_secret.as_bytes();
    let secret_key =
        "secret_key_longer_test_so_no_of_bytes_goes_longre_extra_longer_service_byes".as_bytes();
    let session_layer = SessionLayer::new(store, secret_key).with_secure(false);

    Router::new()
        .route("/", get(home_handler))
        // %%%%%%%%%%  Role Routes  %%%%%%%%%%
        .route("/api/roles", get(roles_handler))
        .route("/api/role/:role_id", get(get_role_handler))
        .route("/api/role", post(create_role_handler))
        .route("/api/role/:role_id", put(put_role_handler))
        .route("/api/role/:role_id", delete(delete_role_handler))
        // %%%%%%%%%%  admin user Routes  %%%%%%%%%%
        .route("/api/admin-users", get(admin_users_handler))
        .route(
            "/api/admin-users/:admin_user_id",
            get(get_admin_user_handler),
        )
        .route(
            "/api/admin-users/:admin_user_id",
            put(put_admin_user_handler),
        )
        .route(
            "/api/admin-users/:admin_user_id",
            delete(delete_admin_user_handler),
        )
        .route("/api/admin-users", post(create_admin_user_handler))
        .route("/admin", get(get_admin_handler))
        // %%%%%%%%%%  middleware Routes  %%%%%%%%%%
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        // .route("/api/auth/login", post(login_admin_user_handler))
        .route("/admin/login", post(post_admin_login_handler))
        .route("/admin/login", get(get_admin_login_handler))
        .route("/test-pp", get(test_pp_handler))
        .nest_service("/public", public_static_service)
        .with_state(app_state)
        .layer(session_layer)
        .layer(cors_layer)
}

pub async fn establish_connection() -> sea_orm::DatabaseConnection {
    let config: Config = Config::new();
    let database_url: String = config.database_url;

    Database::connect(&database_url).await.unwrap()
}

fn translate_key(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new(&format!("param 0")))
        .unwrap();
    let param_value: String = param.value().render();

    let tran = translate(String::from(param_value));

    write!(out, "{}", tran).unwrap();
    Ok(())
}

fn translate(key: String) -> String {
    let config: I18nConfig = I18nConfig {
        locales: &["en", "fr"],
        directory: "locales",
    };
    let r_i18n: I18n = I18n::configure(&config);

    let translated_text = match r_i18n.translations.get("en") {
        Some(language_json) => {
            if language_json.has_key(&key) {
                language_json[&key].to_string()
            } else {
                String::from(&key)
            }
        }
        None => String::from(&key),
    };
    translated_text
}

fn get_validation_message(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new(&format!("param 0")))
        .unwrap();
    let param_value: String = param.value().render();
    write!(
        out,
        "{}",
        String::from(format!("Hello Validation {}", param_value))
    )
    .unwrap();

    Ok(())
}
