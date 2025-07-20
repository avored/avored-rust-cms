use crate::error::Error;
use crate::models::token_claim_model::TokenClaims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;
use tonic::{Request, Status};

pub fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) => {
            let auth_value = t.to_str().map_err(|_e| {
                Status::unavailable("authorization header value is not valid string")
            })?;

            let jwt_token = &env::var("AVORED_JWT_SECRET")
                .map_err(|_| Error::ConfigMissing("AVORED_JWT_SECRET".to_string()))?;
            let token = auth_value
                .strip_prefix("Bearer ")
                .map(|auth| auth.to_owned());
            let claims = decode::<TokenClaims>(
                &token.unwrap_or_default(),
                &DecodingKey::from_secret(jwt_token.as_ref()),
                &Validation::default(),
            )
            .map_err(|_| Status::unauthenticated("No valid auth token claims found"))?
            .claims;
            req.extensions_mut().insert(claims);

            Ok(req)
        }
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
