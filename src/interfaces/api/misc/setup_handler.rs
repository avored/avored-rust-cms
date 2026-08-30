use axum::{extract::State, Json};
use crate::core::domain::entities::user::StorableUser;
use crate::error::Result;
use crate::avored_state::AppState;


pub async fn setup_handler(
    State(state): State<AppState>,
    Json(payload): Json<SetupCommand>,
) -> Result<Json<SetupResponse>> {
    println!("->> {:<12} - setup_handler", "REST_API_HANDLER");

    let storable_user = SetupCommand::from(&payload);
    
    let result = state.misc_use_case.setup(storable_user).await;

    let res = SetupResponse {
        success: true,
    };

    Ok(Json(res))
}



#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SetupCommand {
    pub name: String,
    pub email: String,
    pub password: String
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SetupResponse {
    pub success: bool
}

impl SetupCommand {
    fn from(&self) -> StorableUser {
        StorableUser {
            name: self.name.clone(),
            email: self.email.clone(),
            password: self.password.clone()
        }    
    }
}
