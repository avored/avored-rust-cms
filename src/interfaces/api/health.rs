use crate::avored_state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

#[derive(serde::Serialize)]
pub struct HealthResponse {
    status: &'static str,
    database: &'static str,
    version: &'static str,
}

pub async fn health_check_handler(State(state): State<AppState>) -> impl IntoResponse {
    let (datastore, database_session) = &state.database_provider.db;
    let database_connected = match datastore.execute("RETURN 1;", database_session, None).await {
        Ok(_) => true,
        Err(error) => {
            tracing::warn!(error = ?error, "health check database probe failed");
            false
        }
    };

    health_response(database_connected)
}

fn health_response(database_connected: bool) -> (StatusCode, Json<HealthResponse>) {
    if database_connected {
        (
            StatusCode::OK,
            Json(HealthResponse {
                status: "healthy",
                database: "connected",
                version: env!("CARGO_PKG_VERSION"),
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "unhealthy",
                database: "disconnected",
                version: env!("CARGO_PKG_VERSION"),
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_database_probe_returns_service_unavailable() {
        let (status, Json(body)) = health_response(false);

        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(body.status, "unhealthy");
        assert_eq!(body.database, "disconnected");
    }
}
