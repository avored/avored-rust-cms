use std::sync::Arc;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::application::use_cases::{
        ApiKeyUseCase, AuthUseCase, EmailTemplateUseCase, EntityUseCase, MiscUseCase,
        SettingUseCase, WorkflowUseCase,
    },
    infrastructure::persistence::{
        api_key_repository::ApiKeyRepositoryImpl,
        email_template_repository::EmailTemplateRepositoryImpl,
        entity_repository::EntityRepositoryImpl, misc_repository::MiscRepositoryImpl,
        setting_repository::SettingRepositoryImpl, workflow_repository::WorkflowRepositoryImpl,
        AuthRepositoryImpl,
    },
    infrastructure::workflow_dispatcher::{HttpWebhookExecutor, WorkflowDispatcher},
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

    pub entity_use_case: EntityUseCase<EntityRepositoryImpl>,

    pub email_template_use_case: EmailTemplateUseCase<EmailTemplateRepositoryImpl>,

    pub workflow_use_case: WorkflowUseCase<WorkflowRepositoryImpl>,

    pub workflow_dispatcher: WorkflowDispatcher,

    pub api_key_use_case: ApiKeyUseCase<ApiKeyRepositoryImpl>,

    pub setting_use_case: SettingUseCase<SettingRepositoryImpl>,

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

        let entity_repository = EntityRepositoryImpl::new(avored_database_provider.clone());
        let entity_use_case = EntityUseCase::new(entity_repository);

        let email_template_repository =
            EmailTemplateRepositoryImpl::new(avored_database_provider.clone());
        let email_template_use_case = EmailTemplateUseCase::new(email_template_repository);

        let workflow_repository = WorkflowRepositoryImpl::new(avored_database_provider.clone());
        let workflow_use_case = WorkflowUseCase::new(workflow_repository);
        let workflow_dispatcher =
            WorkflowDispatcher::new(workflow_use_case.clone(), HttpWebhookExecutor::new());

        let api_key_repository = ApiKeyRepositoryImpl::new(avored_database_provider.clone());
        let api_key_use_case = ApiKeyUseCase::new(api_key_repository);

        let setting_repository = SettingRepositoryImpl::new(avored_database_provider.clone());
        let setting_use_case = SettingUseCase::new(setting_repository);

        Ok(Self {
            leptos_options,
            database_provider: avored_database_provider,
            config: Arc::new(config),
            auth_use_case,
            misc_use_case,
            entity_use_case,
            email_template_use_case,
            workflow_use_case,
            workflow_dispatcher,
            api_key_use_case,
            setting_use_case,
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
    let entity_repository = EntityRepositoryImpl::new(database_provider.clone());
    let email_template_repository = EmailTemplateRepositoryImpl::new(database_provider.clone());
    let workflow_repository = WorkflowRepositoryImpl::new(database_provider.clone());
    let workflow_use_case = WorkflowUseCase::new(workflow_repository);
    let workflow_dispatcher =
        WorkflowDispatcher::new(workflow_use_case.clone(), HttpWebhookExecutor::new());
    let api_key_repository = ApiKeyRepositoryImpl::new(database_provider.clone());
    let api_key_use_case = ApiKeyUseCase::new(api_key_repository);
    let setting_repository = SettingRepositoryImpl::new(database_provider.clone());
    let setting_use_case = SettingUseCase::new(setting_repository);

    AppState {
        leptos_options: leptos::config::LeptosOptions::builder()
            .output_name("avored-rust-cms-test")
            .build(),
        auth_use_case: AuthUseCase::new(auth_repository),
        misc_use_case: MiscUseCase::new(misc_repository),
        entity_use_case: EntityUseCase::new(entity_repository),
        email_template_use_case: EmailTemplateUseCase::new(email_template_repository),
        workflow_use_case,
        workflow_dispatcher,
        api_key_use_case,
        setting_use_case,
        database_provider,
        config: Arc::new(AvoRedConfigProvider {
            database_folder: "mem://".to_string(),
            database_name: "auth".to_string(),
            database_namespace: "test".to_string(),
            password_salt: "test-salt".to_string(),
            jwt_secret_key: String::new(),
            cors_allowed_app_url: vec![],
        }),
    }
}
