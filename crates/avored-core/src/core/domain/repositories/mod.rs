// @todo think of renaming to something so we know the different between pub trait and repository implementation itself.
pub mod auth_repository;

pub use auth_repository::AuthRepository;

pub mod misc_repository;

pub use misc_repository::MiscRepository;

pub mod entity_repository;

pub use entity_repository::EntityRepository;

pub mod email_template_repository;
pub use email_template_repository::EmailTemplateRepository;

pub mod workflow_repository;
pub use workflow_repository::WorkflowRepository;

pub mod api_key_repository;
pub use api_key_repository::ApiKeyRepository;

pub mod setting_repository;
pub use setting_repository::SettingRepository;
