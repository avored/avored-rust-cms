use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub iat: usize,
    pub exp: usize,
}