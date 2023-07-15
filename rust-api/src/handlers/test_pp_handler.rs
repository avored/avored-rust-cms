use axum::{
    async_trait,
    extract::{rejection::FormRejection, Form, FromRequest, State},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    Json, RequestExt,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use validator::Validate;

use crate::routes::AppState;

pub async fn test_pp_handler(
    app_state: State<Arc<AppState>>,
    ValidatedForm(input): ValidatedForm<NameInput>,
) -> impl IntoResponse {
    let data = json!({});

    let handlebars = &app_state.handlebars;

    let html = handlebars
        .render("auth/login", &data)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct NameInput {
    #[validate(length(min = 5, message = "Can not be empty"))]
    pub name: String,
}

pub struct ValidatedJson<J>(pub J);

#[async_trait]
impl<S, B, J> FromRequest<S, B> for ValidatedJson<J>
where
    B: Send + 'static,
    S: Send + Sync,
    J: Validate + 'static,
    Json<J>: FromRequest<(), B>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req
            .extract::<Json<J>, _>()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"))?;
        data.validate()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"))?;
        Ok(Self(data))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    B: Send + 'static,
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
