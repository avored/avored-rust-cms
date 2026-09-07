use axum::{Router, http::HeaderValue, routing::{delete, get, post, put}};
use leptos::context::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};

use crate::{
    avored_state::AppState, infrastructure::middleware::auth_middleware, interfaces::web::{shell::Shell, web_routes::WebApp},
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
            "/api/attributes",get(crate::interfaces::api::attribute::paginate_attribute_handler::paginate_attributes_handler),
        )
        .route(
            "/api/attributes",post(crate::interfaces::api::attribute::create_attribute_handler::create_attribute_handler),
        )
        .route(
            "/api/attributes/{id}",get(crate::interfaces::api::attribute::fetch_attribute_handler::fetch_attribute_handler),
        ) 
        
        .route(
            "/api/attributes/{id}",put(crate::interfaces::api::attribute::update_attribute_handler::update_attribute_handler),
        ) 
        .route(
            "/api/attributes/{id}",delete(crate::interfaces::api::attribute::delete_attribute_handler::delete_attribute_handler),
        ) 
        
        .route(
            "/api/entities/option",get(crate::interfaces::api::entity::option_entities_handler::option_entities_handler),
        )
        .route(
            "/api/entities",
            axum::routing::post(crate::interfaces::api::entity::create_entity_handler)
                .get(crate::interfaces::api::entity::paginate_entities_handler),
        )
        .route(
            "/api/entities/{id}",
            axum::routing::get(crate::interfaces::api::entity::fetch_entity_handler)
                .put(crate::interfaces::api::entity::update_entity_handler)
                .delete(crate::interfaces::api::entity::delete_entity_handler),
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
        .nest_service("/public", ServeDir::new(std::path::Path::new("target").join("site")))
        .nest_service("/assets", ServeDir::new(std::path::Path::new("assets")));

    Ok(router.with_state(state))
}
