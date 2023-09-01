pub mod authenticate_admin_user_request;
pub mod store_admin_user_request;
pub mod store_component_request;
pub mod store_role_request;
pub mod update_admin_user_request;
use validator::ValidationErrors;

use crate::providers::avored_session_provider::AvoRedSession;

pub trait ValidateRequest {
    fn validation_error(&self, session: &mut AvoRedSession) -> ValidationErrors;
}
