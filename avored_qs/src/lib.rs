use axum::{extract::{FromRequest, RawForm}, BoxError, async_trait, body::HttpBody, http::{Request, StatusCode}, response::{Response, IntoResponse}, RequestExt};
use serde::de::DeserializeOwned;
use urlencoding::decode_binary;


#[derive(Debug, Clone, Copy, Default)]
pub struct AvoRedForm<T>(pub T);



#[async_trait]
impl<T, S, B> FromRequest<S, B> for AvoRedForm<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AvoRedError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {

        match req.extract().await {
            Ok(RawForm(bytes)) => {

                let decoded = decode_binary(&bytes).into_owned();
                let decoded = String::from_utf8_lossy(&decoded).into_owned();
                match serde_qs::from_str(&decoded) {
                    Ok(value) => Ok(AvoRedForm(value)),
                    Err(_) => Err(AvoRedError::ParseFormError)
                }
            }
            Err(_) => Err(AvoRedError::ParseFormError),
        }
    }
}


pub enum AvoRedError {
    ParseFormError,
}


impl IntoResponse for AvoRedError {
    fn into_response(self) -> Response {
        let status = StatusCode::UNPROCESSABLE_ENTITY;

        let body = "there is an issue while parsing the form data. It might be an syntaxt issue";

        (status, body).into_response()
    }
}
