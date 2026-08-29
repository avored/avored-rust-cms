use std::sync::Arc;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::application::use_cases::AuthUseCase,
    infrastructure::persistence::AuthRepositoryImpl,
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

        Ok(Self {
            leptos_options,
            auth_use_case,
            database_provider: avored_database_provider,
            config: Arc::new(config),
        })
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
