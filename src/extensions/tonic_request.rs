use crate::error::Error;
use crate::models::token_claim_model::TokenClaims;
use tonic::Request;

/// `tonic_request`
pub trait TonicRequest {
    /// The error type returned by the trait methods.
    type Error;

    /// get token claim
    fn get_token_claim(&self) -> crate::error::Result<TokenClaims>;
}

impl<R> TonicRequest for Request<R> {
    type Error = Error;

    fn get_token_claim(&self) -> crate::error::Result<TokenClaims> {
        match self.extensions().get::<TokenClaims>() {
            Some(claims) => Ok(claims.clone()),
            None => Err(Error::Unauthenticated(String::from("token is malformed"))),
        }
    }
}
