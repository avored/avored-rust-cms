use axum::Router;
use axum::response::IntoResponse;
use leptos::config::LeptosOptions;
use leptos::prelude::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use crate::infra::setup::AppState;
use crate::pages::app::App;
use crate::pages::shell::Shell;

pub async fn create_app(state: AppState) -> crate::error::Result<Router> {

    let leptos_options = state.leptos_options.clone();
    // let leptos_options = state.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let greeter = crate::grpc_server::MyGreeter::default();
    let grpc_service =
        crate::grpc_server::helloworld::greeter_server::GreeterServer::new(greeter);

    let router = Router::<AppState>::new()
        .route_service("/helloworld.Greeter/SayHello", grpc_service)
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
            "/pkg",
            ServeDir::new(std::path::Path::new(&*leptos_options.site_root).join("pkg")),
        )
        .nest_service("/public", ServeDir::new(std::path::Path::new("public")))
        
        .fallback(file_and_error_handler)
        ;

    Ok(router.with_state(state))
    
}


pub async fn file_and_error_handler(
    uri: axum::http::Uri,
    axum::extract::State(options): axum::extract::State<LeptosOptions>,
    req: axum::extract::Request,
) -> axum::response::Response {
    log::info!("File and error handler");
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == axum::http::StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(Shell);
        handler(req).await.into_response()
    }
}


pub async fn get_static_file(
    uri: axum::http::Uri,
    root: &str,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let req = axum::extract::Request::builder()
        .uri(uri.clone())
        .body(axum::body::Body::empty())
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
