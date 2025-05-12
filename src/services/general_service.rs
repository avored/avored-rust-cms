use crate::api::proto::admin_user::AdminUserModel as GrpcAdminUserModel;
use crate::api::proto::general::{LoggedInUserResponse};
use crate::models::token_claim_model::TokenClaims;

pub struct GeneralService {
    
}

impl GeneralService {
    pub async fn logged_in_user(
        &self,
        claims: TokenClaims,
    ) -> crate::error::Result<LoggedInUserResponse> {
        let logged_in_user = claims.admin_user_model;
        
        let model: GrpcAdminUserModel = logged_in_user.try_into()?;
        
        let logged_in_user = LoggedInUserResponse {
            status: true,
            data: Some(model)
        };
        
        Ok(logged_in_user)
    }
}

impl GeneralService {
    pub fn new() -> crate::error::Result<GeneralService> {
        Ok(GeneralService {})
    }
}
