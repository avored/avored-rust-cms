use crate::interfaces::api::{
    api_key::{
        create_api_key_handler, delete_api_key_handler, fetch_api_key_handler,
        paginate_api_keys_handler, update_api_key_handler,
    },
    api_manager::{
        create_entity_record_handler::create_entity_record_handler,
        delete_entity_record_handler::delete_entity_record_handler,
        fetch_entity_record_handler::fetch_entity_record_handler,
        paginate_entities_handler::paginate_entities_handler as api_manager_paginate_entities_handler,
        update_entity_record_handler::update_entity_record_handler,
    },
    collection::{
        create_collection_handler::create_collection_handler,
        delete_collection_handler::delete_collection_handler,
        fetch_collection_by_id_handler::fetch_collection_handler,
        paginate_collections_handler::paginate_collections_handler,
        update_collection_handler::update_collection_handler,
    },
    email_template::{
        create_email_template_handler::create_email_template_handler,
        delete_email_template_handler::delete_email_template_handler,
        fetch_email_template_handler::fetch_email_template_handler,
        paginate_email_templates_handler::paginate_email_templates_handler,
        update_email_template_handler::update_email_template_handler,
    },
    entity::{
        create_entity_handler, delete_entity_handler, fetch_entity_handler,
        option_entities_handler::option_entities_handler, paginate_entities_handler,
        update_entity_handler, update_entity_identifier_handler::update_entity_identifier_handler,
    },
    setting::{fetch_settings_handler, update_settings_handler},
    workflow::{
        create_workflow_handler, delete_workflow_handler, fetch_workflow_handler,
        list_workflow_executions_handler, list_workflows_handler, set_workflow_active_handler,
        update_workflow_handler,
    },
};
use axum::{
    http::HeaderValue,
    routing::{delete, get, patch, post, put},
    Router,
};
use leptos::context::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::{
    avored_state::AppState,
    infrastructure::middleware::{api_key_middleware, auth_middleware},
    interfaces::web::{shell::Shell, web_routes::WebApp},
};

pub fn rest_api_routes(state: AppState) -> crate::error::Result<Router> {
    let routes = generate_route_list(WebApp);

    let mut origins: Vec<HeaderValue> = vec![];

    let env_str_allowed_cors = state.config.cors_allowed_app_url.clone();

    for origin in &env_str_allowed_cors {
        origins.push(HeaderValue::from_str(origin)?);
    }

    let cors = CorsLayer::new()
        .allow_origin(origins) // Allow all origins for local development
        .allow_headers(Any) // Allow all headers
        .allow_methods(Any) // Allow all methods
        .expose_headers(Any); // Expose all headers

    let router = Router::<AppState>::new()
        .route("/api/collection", get(paginate_collections_handler))
        .route("/api/collection", post(create_collection_handler))
        .route("/api/email-templates", post(create_email_template_handler))
        .route(
            "/api/email-templates/{id}",
            get(fetch_email_template_handler),
        )
        .route(
            "/api/email-templates/{id}",
            put(update_email_template_handler),
        )
        .route(
            "/api/email-templates/{id}",
            delete(delete_email_template_handler),
        )
        .route(
            "/api/email-templates",
            get(paginate_email_templates_handler),
        )
        .route("/api/workflows", get(list_workflows_handler))
        .route("/api/workflows", post(create_workflow_handler))
        .route("/api/workflows/{id}", get(fetch_workflow_handler))
        .route("/api/workflows/{id}", put(update_workflow_handler))
        .route("/api/workflows/{id}", delete(delete_workflow_handler))
        .route(
            "/api/workflows/{id}/active",
            patch(set_workflow_active_handler),
        )
        .route(
            "/api/workflows/{id}/executions",
            get(list_workflow_executions_handler),
        )
        .route("/api/collection/{record_id}", get(fetch_collection_handler))
        .route(
            "/api/collection/{record_id}",
            put(update_collection_handler),
        )
        .route(
            "/api/collection/{record_id}",
            delete(delete_collection_handler),
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
        .route("/api/api-keys", get(paginate_api_keys_handler))
        .route("/api/api-keys", post(create_api_key_handler))
        .route("/api/api-keys/{id}", get(fetch_api_key_handler))
        .route("/api/api-keys/{id}", put(update_api_key_handler))
        .route("/api/api-keys/{id}", delete(delete_api_key_handler))
        .route(
            "/api/v1/admin/me",
            get(crate::interfaces::api::auth::get_profile_handler),
        )
        .route(
            "/api/v1/admin/me",
            put(crate::interfaces::api::auth::update_profile_handler),
        )
        .route(
            "/api/v1/admin/me/password",
            put(crate::interfaces::api::auth::change_password_handler),
        )
        .route("/api/v1/settings", get(fetch_settings_handler))
        .route("/api/v1/settings", put(update_settings_handler))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware::check_auth,
        ));

    let v1_router = Router::<AppState>::new()
        .route(
            "/api/v1/entities",
            get(api_manager_paginate_entities_handler),
        )
        .route("/api/v1/entities", post(create_entity_record_handler))
        .route("/api/v1/entities/{id}", get(fetch_entity_record_handler))
        .route("/api/v1/entities/{id}", patch(update_entity_record_handler))
        .route(
            "/api/v1/entities/{id}",
            delete(delete_entity_record_handler),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            api_key_middleware::check_api_key_auth,
        ));

    let router = router
        .merge(v1_router)
        // Versioned API Manager routes (public or specific auth if any)
        .route(
            "/api/auth/login",
            axum::routing::post(crate::interfaces::api::auth::login_handler),
        )
        .route(
            "/api/misc/setup",
            axum::routing::post(crate::interfaces::api::misc::setup_handler::setup_handler),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::debug_span!(
                        "http_request",
                        method = %request.method(),
                        path = %request.uri().path()
                    )
                })
                .on_request(DefaultOnRequest::new().level(tracing::Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(tracing::Level::DEBUG)),
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
