use crate::api::proto::cms::cms_server::Cms;
use crate::api::proto::cms::{
    GetCmsContentRequest, GetCmsContentResponse, SentContactFormRequest, SentContactFormResponse,
};
use crate::avored_state::AvoRedState;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

/// AvoRed CMS API
pub struct CmsApi {
    /// The AvoRed state containing services and configurations
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Cms for CmsApi {
    async fn get_cms_content(
        &self,
        request: Request<GetCmsContentRequest>,
    ) -> Result<Response<GetCmsContentResponse>, Status> {
        println!("->> {:<12} - get_cms_content", "gRPC_Cms_Api_Service");

        // let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        // let logged_in_user = claims.admin_user_model;
        //
        // let has_permission_bool = self.state
        //     .admin_user_service
        //     .has_permission(logged_in_user, String::from("get_cms_content"))
        //     .await?;
        // if !has_permission_bool {
        //     let status = Status::permission_denied("You don't have permission to access this resource");
        //     return Err(status);
        // }

        let req = request.into_inner();

        match self
            .state
            .cms_service
            .get_cms_content(req, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn sent_contact_form(
        &self,
        request: Request<SentContactFormRequest>,
    ) -> Result<Response<SentContactFormResponse>, Status> {
        let req = request.into_inner();

        match self
            .state
            .cms_service
            .sent_contact_form(&self.state.template, req)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
