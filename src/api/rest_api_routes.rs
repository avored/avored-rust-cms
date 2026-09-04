use axum::Router;
use leptos::context::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::services::ServeDir;

use crate::{
    avored_state::AppState, interfaces::web::{shell::Shell, web_routes::WebApp},
};

pub fn rest_api_routes(state: AppState) -> crate::error::Result<Router> {
    let routes = generate_route_list(WebApp);

    let router = Router::<AppState>::new()
        .route(
            "/api/auth/login",
            axum::routing::post(crate::interfaces::api::auth::login_handler),
        )
        .route(
            "/api/misc/setup",
            axum::routing::post(crate::interfaces::api::misc::setup_handler::setup_handler),
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
