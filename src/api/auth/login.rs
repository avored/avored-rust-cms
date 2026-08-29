#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUser,
    pub authenticated: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub fn build_demo_login_response() -> LoginResponse {
    LoginResponse {
        token: "demo-token".to_string(),
        user: LoginUser {
            id: "demo-user-id".to_string(),
            name: "Demo User".to_string(),
            email: "demo@avored.local".to_string(),
        },
        authenticated: true,
    }
}

pub async fn handle_login() -> axum::Json<LoginResponse> {
    axum::Json(build_demo_login_response())
}

#[cfg(test)]
mod tests {
    use super::build_demo_login_response;

    #[test]
    fn demo_login_response_is_authenticated() {
        let response = build_demo_login_response();
        assert!(response.authenticated);
        assert_eq!(response.user.name, "Demo User");
        assert_eq!(response.user.email, "demo@avored.local");
    }
}
