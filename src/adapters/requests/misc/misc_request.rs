use crate::{domain::models::admin_user::StorableAdminUser, infra::grpc::misc::SetupRequest};
use crate::error::Error;
use crate::error::Result;

impl TryFrom<SetupRequest> for StorableAdminUser {
    type Error = Error;
    fn try_from(val: SetupRequest) -> Result<StorableAdminUser> {
        Ok(StorableAdminUser {
            full_name: val.full_name,
            email: val.email,
            password_hash: val.password,
            logged_in_user: String::from(""),
            profile_image: String::from(""),
            is_super_admin: false,
        })
    }
}
