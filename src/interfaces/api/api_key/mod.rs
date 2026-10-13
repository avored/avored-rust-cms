use crate::core::application::dtos::api_key_dto::ApiKeyResponse;
use crate::core::domain::entities::api_key::ApiKeyModel;

pub mod paginate_api_keys_handler;
pub use paginate_api_keys_handler::paginate_api_keys_handler;

pub mod create_api_key_handler;
pub use create_api_key_handler::create_api_key_handler;

pub mod fetch_api_key_handler;
pub use fetch_api_key_handler::fetch_api_key_handler;

pub mod update_api_key_handler;
pub use update_api_key_handler::update_api_key_handler;

pub mod delete_api_key_handler;
pub use delete_api_key_handler::delete_api_key_handler;

pub(crate) fn api_key_model_to_response(model: ApiKeyModel) -> ApiKeyResponse {
    ApiKeyResponse {
        id: model.id,
        name: model.name,
        key_prefix: model.key_prefix,
        permissions: model.permissions,
        created_at: model.created_at,
        updated_at: model.updated_at,
        plain_secret: None,
    }
}
