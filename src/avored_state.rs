use std::sync::Arc;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::application::use_cases::{AuthUseCase, MiscUseCase},
    infrastructure::persistence::{misc_repository::MiscRepositoryImpl, AuthRepositoryImpl},
    providers::{
        avored_config_provider::AvoRedConfigProvider,
        avored_database_provider::AvoRedDatabaseProvider,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,

    /// Auth use case with SurrealDB persistence repository
    pub auth_use_case: AuthUseCase<AuthRepositoryImpl>,

    pub misc_use_case: MiscUseCase<MiscRepositoryImpl>,

    /// Database provider for `AvoRed` (SurrealDB).
    pub database_provider: Arc<AvoRedDatabaseProvider>,

    /// Configuration provider for `AvoRed`.
    pub config: Arc<AvoRedConfigProvider>,
}

impl AppState {
    pub async fn new(leptos_options: LeptosOptions) -> crate::error::Result<Self> {
        let config = AvoRedConfigProvider::new()?;

        let avored_database_provider = Arc::new(
            AvoRedDatabaseProvider::register(
                &config.database_folder,
                &config.database_namespace,
                &config.database_name,
            )
            .await?,
        );

        let auth_repository = AuthRepositoryImpl::new(avored_database_provider.clone());
        let auth_use_case = AuthUseCase::new(auth_repository);

        let misc_repository = MiscRepositoryImpl::new(avored_database_provider.clone());
        let misc_use_case = MiscUseCase::new(misc_repository);

        Ok(Self {
            leptos_options,
            database_provider: avored_database_provider,
            config: Arc::new(config),
            auth_use_case,
            misc_use_case,
        })
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}



pub async fn test_avored_state() -> AppState {
    let auth_repository =
        crate::infrastructure::persistence::auth_repository::test_auth_repository().await;
    let database_provider = auth_repository.database_provider.clone();
    let misc_repository = MiscRepositoryImpl::new(database_provider.clone());

    AppState {
        leptos_options: leptos::config::LeptosOptions::builder()
            .output_name("avored-rust-cms-test")
            .build(),
        auth_use_case: AuthUseCase::new(auth_repository),
        misc_use_case: MiscUseCase::new(misc_repository),
        database_provider,
        config: Arc::new(AvoRedConfigProvider {
            database_folder: "mem://".to_string(),
            database_name: "auth".to_string(),
            database_namespace: "test".to_string(),
            password_salt: String::new(),
            jwt_secret_key: String::new(),
            cors_allowed_app_url: vec![],
        }),
    }
}
