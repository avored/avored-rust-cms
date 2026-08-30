use std::sync::Arc;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::application::use_cases::{AuthUseCase, MiscUseCase}, infrastructure::persistence::{AuthRepositoryImpl, misc_repository::MiscRepositoryImpl}, providers::{
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
            misc_use_case
        })
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
