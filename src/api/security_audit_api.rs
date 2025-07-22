use crate::api::proto::security_audit::security_audit_server::SecurityAudit;
use crate::api::proto::security_audit::{
    CreateSecurityAuditRequest, CreateSecurityAuditResponse, DeleteSecurityAuditRequest,
    DeleteSecurityAuditResponse, GetIpSecuritySummaryRequest, GetIpSecuritySummaryResponse,
    GetSecurityAuditRequest, GetSecurityAuditResponse, GetSecurityAuditsByIpRequest,
    GetSecurityAuditsByIpResponse, GetSecurityAuditsByUserRequest, GetSecurityAuditsByUserResponse,
    GetSecurityAuditsPaginatedRequest, GetSecurityAuditsPaginatedResponse, LogSecurityEventRequest,
    LogSecurityEventResponse, UpdateSecurityAuditRequest, UpdateSecurityAuditResponse,
};
use crate::avored_state::AvoRedState;
use crate::error::Error::Tonic;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

pub struct SecurityAuditApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl SecurityAudit for SecurityAuditApi {
    async fn create_security_audit(
        &self,
        request: Request<CreateSecurityAuditRequest>,
    ) -> Result<Response<CreateSecurityAuditResponse>, Status> {
        println!(
            "->> {:<12} - create_security_audit",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("create_security_audit"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .create_audit_from_grpc(req, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn log_security_event(
        &self,
        request: Request<LogSecurityEventRequest>,
    ) -> Result<Response<LogSecurityEventResponse>, Status> {
        println!(
            "->> {:<12} - log_security_event",
            "gRPC_SecurityAudit_Service"
        );

        // Security events can be logged without authentication for system-level events
        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .log_security_event_from_grpc(req, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_security_audit(
        &self,
        request: Request<GetSecurityAuditRequest>,
    ) -> Result<Response<GetSecurityAuditResponse>, Status> {
        println!(
            "->> {:<12} - get_security_audit",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_security_audit"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .get_audit_by_id_grpc(req.id, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_security_audits_by_user(
        &self,
        request: Request<GetSecurityAuditsByUserRequest>,
    ) -> Result<Response<GetSecurityAuditsByUserResponse>, Status> {
        println!(
            "->> {:<12} - get_security_audits_by_user",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_security_audits_by_user"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .get_audits_by_admin_user_grpc(
                req.admin_user_id,
                req.page,
                req.per_page,
                &self.state.db,
            )
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_security_audits_by_ip(
        &self,
        request: Request<GetSecurityAuditsByIpRequest>,
    ) -> Result<Response<GetSecurityAuditsByIpResponse>, Status> {
        println!(
            "->> {:<12} - get_security_audits_by_ip",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_security_audits_by_ip"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .get_audits_by_ip_address_grpc(req.ip_address, req.page, req.per_page, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_security_audits_paginated(
        &self,
        request: Request<GetSecurityAuditsPaginatedRequest>,
    ) -> Result<Response<GetSecurityAuditsPaginatedResponse>, Status> {
        println!(
            "->> {:<12} - get_security_audits_paginated",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_security_audits_paginated"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .get_audits_paginated_grpc(req.page, req.per_page, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn update_security_audit(
        &self,
        request: Request<UpdateSecurityAuditRequest>,
    ) -> Result<Response<UpdateSecurityAuditResponse>, Status> {
        println!(
            "->> {:<12} - update_security_audit",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("update_security_audit"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .update_audit_grpc(req.id, req.audit, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn delete_security_audit(
        &self,
        request: Request<DeleteSecurityAuditRequest>,
    ) -> Result<Response<DeleteSecurityAuditResponse>, Status> {
        println!(
            "->> {:<12} - delete_security_audit",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("delete_security_audit"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .delete_audit_grpc(req.id, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_ip_security_summary(
        &self,
        request: Request<GetIpSecuritySummaryRequest>,
    ) -> Result<Response<GetIpSecuritySummaryResponse>, Status> {
        println!(
            "->> {:<12} - get_ip_security_summary",
            "gRPC_SecurityAudit_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_ip_security_summary"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_audit_service
            .get_ip_security_summary_grpc(req.ip_address, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(boxed_status) => Err(*boxed_status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
