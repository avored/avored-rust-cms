use crate::{models::admin_user_model::AdminUserModel, services::admin_user_service::AdminUserService};


// Usually we don't postfix the extension, 
// we already have two admin user model struct so we postfix this as with extension.
pub trait AdminUserModelExtension {
    async fn check_user_has_resouce_access(
        &self,
        admin_user_service: &AdminUserService,
        permission_identifier: String
    ) -> crate::error::Result<()>;
}


impl AdminUserModelExtension for AdminUserModel {
    async fn check_user_has_resouce_access(&self, admin_user_service: &AdminUserService, permission_identifier: String) -> crate::error::Result<()> {
        
        let logged_in_user = self.clone();
         let has_permission_bool = admin_user_service
            .has_permission(logged_in_user, permission_identifier.clone())
            .await?;
        if !has_permission_bool {
            return Err(crate::error::Error::Unauthorizeed(permission_identifier));
        }

        Ok(())
    }
}