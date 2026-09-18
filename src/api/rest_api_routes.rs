use crate::interfaces::api::{
    collection::paginate_collections_handler::paginate_collections_handler,
    entity::{
        create_entity_handler, delete_entity_handler, fetch_entity_handler,
        option_entities_handler::option_entities_handler, paginate_entities_handler,
        update_entity_handler, update_entity_identifier_handler::update_entity_identifier_handler,
    },
};
use axum::{
    http::HeaderValue,
    routing::{delete, get, post, put},
    Router,
};
use leptos::context::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::{
    avored_state::AppState,
    infrastructure::middleware::auth_middleware,
    interfaces::web::{shell::Shell, web_routes::WebApp},
};

pub fn rest_api_routes(state: AppState) -> crate::error::Result<Router> {
    let routes = generate_route_list(WebApp);

    let mut origins: Vec<HeaderValue> = vec![];

    let env_str_allowed_cors = state.config.cors_allowed_app_url.clone();

    for origin in &env_str_allowed_cors {
        origins.push(HeaderValue::from_str(origin).unwrap());
    }

    let cors = CorsLayer::new()
        .allow_origin(origins) // Allow all origins for local development
        .allow_headers(Any) // Allow all headers
        .allow_methods(Any) // Allow all methods
        .expose_headers(Any); // Expose all headers

    let router = Router::<AppState>::new()
        .route(
            "/api/collection/{entity_id}",
            get(paginate_collections_handler),
        )
        .route("/api/entities/options", get(option_entities_handler))
        .route("/api/entities", post(create_entity_handler))
        .route("/api/entities", get(paginate_entities_handler))
        .route("/api/entities/{id}", get(fetch_entity_handler))
        .route("/api/entities/{id}", put(update_entity_handler))
        .route("/api/entities/{id}", delete(delete_entity_handler))
        .route(
            "/api/entities/{id}/identifier",
            put(update_entity_identifier_handler),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware::check_auth,
        ))
        .route(
            "/api/auth/login",
            axum::routing::post(crate::interfaces::api::auth::login_handler),
        )
        .route(
            "/api/misc/setup",
            axum::routing::post(crate::interfaces::api::misc::setup_handler::setup_handler),
        )
        /* frontend routes */
        .layer(cors)
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let state = state.clone();
                move || {
                    provide_context(state.clone());
                }
            },
            Shell,
        )
        .nest_service(
            "/public",
            ServeDir::new(std::path::Path::new("target").join("site")),
        )
        .nest_service("/assets", ServeDir::new(std::path::Path::new("assets")));

    Ok(router.with_state(state))
}
