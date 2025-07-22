use crate::api::proto::security_audit::security_alert_server::SecurityAlert;
use crate::api::proto::security_audit::{
    CreateSecurityAlertAutoIdRequest, CreateSecurityAlertAutoIdResponse,
    CreateSecurityAlertRequest, CreateSecurityAlertResponse, DeleteSecurityAlertRequest,
    DeleteSecurityAlertResponse, GetAlertStatisticsRequest, GetAlertStatisticsResponse,
    GetAlertsBySourceRequest, GetAlertsBySourceResponse, GetAlertsByTypeRequest,
    GetAlertsByTypeResponse, GetCriticalUnresolvedAlertsRequest,
    GetCriticalUnresolvedAlertsResponse, GetSecurityAlertRequest, GetSecurityAlertResponse,
    GetSecurityAlertsPaginatedRequest, GetSecurityAlertsPaginatedResponse,
    GetUnresolvedAlertsBySeverityRequest, GetUnresolvedAlertsBySeverityResponse,
    ResolveSecurityAlertRequest, ResolveSecurityAlertResponse,
};
use crate::avored_state::AvoRedState;
use crate::error::Error::Tonic;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

pub struct SecurityAlertApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl SecurityAlert for SecurityAlertApi {
    async fn create_security_alert(
        &self,
        request: Request<CreateSecurityAlertRequest>,
    ) -> Result<Response<CreateSecurityAlertResponse>, Status> {
        println!(
            "->> {:<12} - create_security_alert",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("create_security_alert"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .create_alert_from_grpc(req, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn create_security_alert_auto_id(
        &self,
        request: Request<CreateSecurityAlertAutoIdRequest>,
    ) -> Result<Response<CreateSecurityAlertAutoIdResponse>, Status> {
        println!(
            "->> {:<12} - create_security_alert_auto_id",
            "gRPC_SecurityAlert_Service"
        );

        // Auto-ID alerts can be created by system processes without authentication
        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .create_alert_auto_id_from_grpc(req, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_security_alert(
        &self,
        request: Request<GetSecurityAlertRequest>,
    ) -> Result<Response<GetSecurityAlertResponse>, Status> {
        println!(
            "->> {:<12} - get_security_alert",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_security_alert"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .get_alert_by_id_grpc(req.id, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_unresolved_alerts_by_severity(
        &self,
        request: Request<GetUnresolvedAlertsBySeverityRequest>,
    ) -> Result<Response<GetUnresolvedAlertsBySeverityResponse>, Status> {
        println!(
            "->> {:<12} - get_unresolved_alerts_by_severity",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_unresolved_alerts_by_severity"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .get_unresolved_alerts_by_severity_grpc(
                req.severity,
                req.page,
                req.per_page,
                &self.state.db,
            )
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_alerts_by_type(
        &self,
        request: Request<GetAlertsByTypeRequest>,
    ) -> Result<Response<GetAlertsByTypeResponse>, Status> {
        println!(
            "->> {:<12} - get_alerts_by_type",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_alerts_by_type"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .get_alerts_by_type_grpc(req.alert_type, req.page, req.per_page, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_alerts_by_source(
        &self,
        request: Request<GetAlertsBySourceRequest>,
    ) -> Result<Response<GetAlertsBySourceResponse>, Status> {
        println!(
            "->> {:<12} - get_alerts_by_source",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_alerts_by_source"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .get_alerts_by_source_grpc(req.source, req.page, req.per_page, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn resolve_security_alert(
        &self,
        request: Request<ResolveSecurityAlertRequest>,
    ) -> Result<Response<ResolveSecurityAlertResponse>, Status> {
        println!(
            "->> {:<12} - resolve_security_alert",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("resolve_security_alert"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .resolve_alert_grpc(req.id, req.resolved_by, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_security_alerts_paginated(
        &self,
        request: Request<GetSecurityAlertsPaginatedRequest>,
    ) -> Result<Response<GetSecurityAlertsPaginatedResponse>, Status> {
        println!(
            "->> {:<12} - get_security_alerts_paginated",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_security_alerts_paginated"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .get_alerts_paginated_grpc(req.page, req.per_page, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_critical_unresolved_alerts(
        &self,
        request: Request<GetCriticalUnresolvedAlertsRequest>,
    ) -> Result<Response<GetCriticalUnresolvedAlertsResponse>, Status> {
        println!(
            "->> {:<12} - get_critical_unresolved_alerts",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_critical_unresolved_alerts"),
            )
            .await?;

        match self
            .state
            .security_alert_service
            .get_critical_unresolved_alerts_grpc(&self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn delete_security_alert(
        &self,
        request: Request<DeleteSecurityAlertRequest>,
    ) -> Result<Response<DeleteSecurityAlertResponse>, Status> {
        println!(
            "->> {:<12} - delete_security_alert",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("delete_security_alert"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .security_alert_service
            .delete_alert_grpc(req.id, &self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_alert_statistics(
        &self,
        request: Request<GetAlertStatisticsRequest>,
    ) -> Result<Response<GetAlertStatisticsResponse>, Status> {
        println!(
            "->> {:<12} - get_alert_statistics",
            "gRPC_SecurityAlert_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_alert_statistics"),
            )
            .await?;

        match self
            .state
            .security_alert_service
            .get_alert_statistics_grpc(&self.state.db)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
