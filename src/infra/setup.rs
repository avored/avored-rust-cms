use crate::error::Result;
use axum::extract::FromRef;
use leptos::config::{get_configuration, LeptosOptions};
use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};

pub async fn init_app_state() -> Result<AppState> {
    init_log();
    let db = init_database().await?;
    // rust_i18n::i18n!("resources/locales");

    let conf = get_configuration(None)?;
    // let addr = conf.leptos_options.site_addr;

    Ok(AppState {
        leptos_options: conf.leptos_options,
        db,
    })
}

pub type DB = Surreal<Db>;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: DB,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.leptos_options.clone()
    }
}

async fn init_database() -> Result<DB> {
    let db_path = "data/avored.db";
    let db = Surreal::new::<RocksDb>(db_path).await?;

    db.use_ns("public").use_db("avored").await?;

    println!("connected to surrealdb at: {}", db_path);

    Ok(db)
}

fn init_log() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    let log_dir = env::var("AVORED_LOG_DIR").unwrap_or_else(|_| "public/log".to_string());
    let log_file = env::var("AVORED_LOG_FILE").unwrap_or_else(|_| "avored.log".to_string());
    let file = File::create(Path::new(&log_dir).join(&log_file));
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {error:?}"),
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
