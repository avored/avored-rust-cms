//! # AvoRed Rust CMS
//!
//! A secure, high-performance Content Management System built with Rust.
//!
//! This CMS provides enterprise-grade security features including:
//! - Zero-trust security architecture
//! - Comprehensive input validation and injection prevention
//! - LDAP authentication with rate limiting
//! - Real-time security monitoring and alerting
//! - Audit logging and compliance reporting
//!
//! ## Security Features
//!
//! - **Input Validation**: Prevents SQL, LDAP, XSS, and command injection attacks
//! - **Authentication**: Multi-provider support (LDAP, local, OAuth, SAML)
//! - **Authorization**: Role-based access control with fine-grained permissions
//! - **Monitoring**: Real-time threat detection and security health scoring
//! - **Audit**: Comprehensive logging of all security-relevant events
//!
//! ## Architecture
//!
//! The system is built with a modular architecture featuring:
//! - gRPC API services for high-performance communication
//! - Async/await throughout for optimal resource utilization
//! - Comprehensive error handling and recovery
//! - Configurable security policies and thresholds

use crate::api::admin_user_api::AdminUserApi;
use crate::api::asset_api::AssetApi;
use crate::api::auth_api::AuthApi;
use crate::api::cms_api::CmsApi;
use crate::api::content_api::ContentApi;
use crate::api::dashboard_api::DashboardApi;
use crate::api::general_api::GeneralApi;
use crate::api::handlers::asset::store_asset_api_handler::store_asset_api_handler;
use crate::api::misc_api::MiscApi;
use crate::api::proto::admin_user::admin_user_server::AdminUserServer;
use crate::api::proto::asset::asset_server::AssetServer;
use crate::api::proto::auth::auth_server::AuthServer;
use crate::api::proto::cms::cms_server::CmsServer;
use crate::api::proto::content::content_server::ContentServer;
use crate::api::proto::dashboard::dashboard_server::DashboardServer;
use crate::api::proto::general::general_service_server::GeneralServiceServer;
use crate::api::proto::misc::misc_server::MiscServer;
use crate::api::proto::setting::setting_server::SettingServer;
use crate::api::setting_api::SettingApi;
use crate::avored_state::AvoRedState;
use crate::error::Error;
use crate::middleware::grpc_auth_middleware::check_auth;
use crate::middleware::require_jwt_authentication::require_jwt_authentication;
use crate::middleware::security_headers::add_security_headers;
use crate::security::invariants::{RuntimeSecurityMonitor, SecurityInvariantChecker};
use axum::http::HeaderValue;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use axum_tonic::{NestTonic, RestGrpcService};
use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};

mod api;
mod avored_state;
mod error;
mod extensions;
mod middleware;
mod models;
mod providers;
mod repositories;
mod requests;
mod security;
mod services;

const PER_PAGE: u64 = 10;

rust_i18n::i18n!("resources/locales");

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, AvoRed content management system!</h1>")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_log();

    // Perform security invariant checks at startup
    if let Err(e) = SecurityInvariantChecker::check_all_invariants() {
        eprintln!("Security invariant check failed: {}", e);
        return Err(e);
    }
    println!("✅ Security invariant checks passed");

    let state = Arc::new(AvoRedState::new().await?);

    // Start background security monitoring
    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(300)); // Every 5 minutes
        loop {
            interval.tick().await;
            match RuntimeSecurityMonitor::perform_security_health_check().await {
                Ok(report) => {
                    let overall_status = report.overall_status();
                    match overall_status {
                        crate::security::invariants::SecurityStatus::Healthy => {
                            tracing::debug!("Security health check: All systems healthy");
                        }
                        crate::security::invariants::SecurityStatus::Warning => {
                            tracing::warn!("Security health check: Warning status detected");
                        }
                        crate::security::invariants::SecurityStatus::Critical => {
                            tracing::error!("Security health check: Critical issues detected");
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Security health check failed: {}", e);
                }
            }
        }
    });

    let mut origins: Vec<HeaderValue> = vec![];
    for origin in &state.config.cors_allowed_app_url {
        origins.push(HeaderValue::from_str(origin).unwrap());
    }

    // const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
    // const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    //     ["grpc-status", "grpc-message", "grpc-status-details-bin"];
    // const DEFAULT_ALLOW_HEADERS: [&str; 5] =
    //     ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout", "authorization"];

    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins for local development
        .allow_headers(Any) // Allow all headers
        .allow_methods(Any) // Allow all methods
        .expose_headers(Any); // Expose all headers

    let misc_api = MiscApi {
        state: state.clone(),
    };
    let misc_server = MiscServer::new(misc_api);

    let cms_api = CmsApi {
        state: state.clone(),
    };
    let cms_server = CmsServer::new(cms_api);

    let dashboard_api = DashboardApi {
        state: state.clone(),
    };
    let dashboard_server = DashboardServer::with_interceptor(dashboard_api, check_auth);

    let auth_api = AuthApi {
        state: state.clone(),
    };
    let auth_server = AuthServer::new(auth_api);

    let admin_user_api = AdminUserApi {
        state: state.clone(),
    };
    let admin_user_server = AdminUserServer::with_interceptor(admin_user_api, check_auth);

    let content_api = ContentApi {
        state: state.clone(),
    };
    let content_server = ContentServer::with_interceptor(content_api, check_auth);

    let setting_api = SettingApi {
        state: state.clone(),
    };
    let setting_server = SettingServer::with_interceptor(setting_api, check_auth);

    let general_api = GeneralApi {
        state: state.clone(),
    };
    let general_server = GeneralServiceServer::with_interceptor(general_api, check_auth);

    let asset_api = AssetApi {
        state: state.clone(),
    };
    let asset_server = AssetServer::with_interceptor(asset_api, check_auth);

    let grpc_router = Router::new()
        .nest_tonic(misc_server)
        .nest_tonic(auth_server)
        .nest_tonic(dashboard_server)
        .nest_tonic(admin_user_server)
        .nest_tonic(content_server)
        .nest_tonic(setting_server)
        .nest_tonic(general_server)
        .nest_tonic(asset_server)
        .nest_tonic(cms_server)
        .layer(cors.clone())
        .layer(axum::middleware::from_fn(add_security_headers));

    let static_routing_service = ServeDir::new("public");

    let rest_router = Router::new()
        .route("/", get(handler))
        .route("/api/asset", post(store_asset_api_handler))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            require_jwt_authentication,
        ))
        .nest_service("/public", static_routing_service)
        .with_state(state)
        .layer(cors)
        .layer(axum::middleware::from_fn(add_security_headers));

    let service = RestGrpcService::new(rest_router, grpc_router);

    let port = env::var("AVORED_PORT").unwrap_or_else(|_| "50051".to_string());
    let host = env::var("AVORED_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    println!(r"     _             ____          _ ");
    println!(r"    / \__   _____ |  _ \ ___  __| |");
    println!(r"   / _ \ \ / / _ \| |_) / _ \/ _` |");
    println!(r"  / ___ \ V / (_) |  _ <  __/ (_| |");
    println!(r" /_/   \_\_/ \___/|_| \_\___|\__,_|");

    println!();
    println!();

    println!("Server started: http://{}:{}", host, port);

    axum::serve(listener, service.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn init_log() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    let log_dir = env::var("AVORED_LOG_DIR").unwrap_or_else(|_| "public/log".to_string());
    let log_file = env::var("AVORED_LOG_FILE").unwrap_or_else(|_| "avored.log".to_string());
    let file = File::create(Path::new(&log_dir).join(&log_file));
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    // A layer that collects metrics using specific events.
    let metrics_layer = /* ... */ filter::LevelFilter::INFO;

    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
                // Add a filter to *both* layers that rejects spans and
                // events whose targets start with `metrics`.
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("metrics")
                })),
        )
        .with(
            // Add a filter to the metrics label that *only* enables
            // events whose targets start with `metrics`.
            metrics_layer.with_filter(filter::filter_fn(|metadata| {
                metadata.target().starts_with("metrics")
            })),
        )
        .init();
}
