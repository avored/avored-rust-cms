pub mod auth_dto;

pub use auth_dto::{LoginCommand, LoginResult};

pub mod entity_dto;
pub use entity_dto::{
    CreateEntityCommand, EntityPaginationResponse, EntityResponse, UpdateEntityCommand,
};

pub mod collection_dto;

pub mod email_template_dto;
pub use email_template_dto::{EmailTemplatePaginationResponse, PaginateEmailTemplateCommand};

pub mod api_manager_dto;
