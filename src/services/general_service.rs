use crate::api::proto::admin_user::AdminUserModel as GrpcAdminUserModel;
use crate::api::proto::dashboard::VisitByYear as GrpcVisitByYear;
use crate::api::proto::dashboard::VisitByContentType as GrpcVisitByContentType;
use crate::api::proto::general::LoggedInUserResponse;
use crate::error::Result;
use crate::models::token_claim_model::TokenClaims;
use crate::providers::avored_database_provider::DB;
use crate::repositories::visitor_log_repository::VisitorLogRepository;

/// general service
pub struct GeneralService {
    visitor_log_repository: VisitorLogRepository
}

impl GeneralService {
    /// logged in user
    pub async fn logged_in_user(
        &self,
        claims: TokenClaims,
    ) -> Result<LoggedInUserResponse> {
        let logged_in_user = claims.admin_user_model;

        let model: GrpcAdminUserModel = logged_in_user.try_into()?;

        let logged_in_user = LoggedInUserResponse {
            status: true,
            data: Some(model),
        };

        Ok(logged_in_user)
    }
}

impl GeneralService {
    /// new instance for general service
    pub const fn new(visitor_log_repository: VisitorLogRepository) -> Result<Self> {
        Ok(Self {visitor_log_repository})
    }

    /// Get a connection from the pool or create a new one
    pub async fn get_visit_by_year(
        &self,
        (datastore, database_session): &DB,
        year: i64
    ) -> Result<Vec<GrpcVisitByYear>> {
        
        let visits_by_year_list = self
            .visitor_log_repository
            .get_visit_by_year(datastore, database_session, year)
            .await?;

        let mut grpc_visits_by_year_list = vec![];
        

        for number in 1..=12 {
            match visits_by_year_list.clone().iter().find(|x| x.month_number == number) {
                Some(visit) => {
                    let model: GrpcVisitByYear = visit.clone().try_into().unwrap();
                    grpc_visits_by_year_list.push(model);
                },
                None => {
                    let month_name = match number {
                        1 => "Jan",
                        2 => "Feb",
                        3 => "Mar",
                        4 => "Apr",
                        5 => "May",
                        6 => "Jun",
                        7 => "Jul",
                        8 => "Aug",
                        9 => "Sep",
                        10 => "Oct",
                        11 => "Nov",
                        12 => "Dec",
                        _ => "",
                    };
                    let empty_model = GrpcVisitByYear {
                        visits: 0,
                        month: month_name.to_string()
                    };
                    grpc_visits_by_year_list.push(empty_model);
                }
                
            }
        }

        Ok(grpc_visits_by_year_list)
    }

    /// Get a connection from the pool or create a new one
    pub async fn get_visit_by_content_type(
        &self,
        (datastore, database_session): &DB,
        content_type: String,
        year: i64
    ) -> Result<Vec<GrpcVisitByContentType>> {
                
        let visits_by_content_type_list = self
            .visitor_log_repository
            .get_visit_by_content_type(datastore, database_session, content_type.clone(), year)
            .await?;

        let mut grpc_visits_by_content_type_list = vec![];
        

        for number in 1..=12 {
            match visits_by_content_type_list.clone().iter().find(|x| x.month_number == number) {
                Some(visit) => {
                    let mut model: GrpcVisitByContentType = visit.clone().try_into().unwrap();
                    model.content_type = content_type.clone();
                    grpc_visits_by_content_type_list.push(model);
                },
                None => {
                    let month_name = match number {
                        1 => "Jan",
                        2 => "Feb",
                        3 => "Mar",
                        4 => "Apr",
                        5 => "May",
                        6 => "Jun",
                        7 => "Jul",
                        8 => "Aug",
                        9 => "Sep",
                        10 => "Oct",
                        11 => "Nov",
                        12 => "Dec",
                        _ => "",
                    };
                    let empty_model = GrpcVisitByContentType {
                        visits: 0,
                        month: month_name.to_string(),
                        content_type: content_type.clone()
                    };
                    grpc_visits_by_content_type_list.push(empty_model);
                }
                
            }
        }

        Ok(grpc_visits_by_content_type_list)
    }

}
