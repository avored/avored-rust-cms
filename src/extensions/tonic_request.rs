use tonic::Request;
use crate::error::Error;
use crate::models::token_claim_model::TokenClaims;

pub trait TonicRequest {
    type Error;
    fn get_token_claim(
        &self
    ) -> crate::error::Result<TokenClaims>;
}


impl<R> TonicRequest for Request<R> {
    type Error = Error;

    fn get_token_claim(&self) -> crate::error::Result<TokenClaims> { 
        match self.extensions().get::<TokenClaims>() {
            Some(claims) => Ok(claims.clone()),
            None => Err(Error::Unauthenticated(String::from("token is malformed")))
        }
    }
}