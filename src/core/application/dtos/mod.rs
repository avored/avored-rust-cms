pub mod auth_dto;

pub use auth_dto::{LoginCommand, LoginResult};

pub mod entity_dto;
pub use entity_dto::{
    CreateEntityCommand, EntityPaginationResponse, EntityResponse, UpdateEntityCommand,
};
