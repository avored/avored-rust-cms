use crate::grpc_auth::LoginRequest;

impl LoginRequest {
    pub fn validate(&self) -> crate::error::Result<(bool, String)> {}

}