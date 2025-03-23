use jsonwebtoken::{decode, DecodingKey, Validation};
use tonic::{Request, Status};
use crate::models::token_claim_model::TokenClaims;

pub fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {

    match req.metadata().get("authorization") {
        Some(t) => {
            let auth_value = t.to_str().map_err(|_e| Status::unavailable("authorization header value is not valid string"))?;
            let secret = "oikcvnmlmgumgxfhccfpsjhpazbvfazdojemqherghvseoedu";

            let token = match auth_value.strip_prefix("Bearer ") {
                Some(auth) => Some(auth.to_owned()),
                _ => None,
            };
            let claims = decode::<TokenClaims>(
                &token.unwrap_or_default(),
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            ).map_err(|_| {
                Status::unauthenticated("No valid auth token claims found")
            })?
                .claims;
            req.extensions_mut().insert(claims);

            Ok(req)
        },
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
