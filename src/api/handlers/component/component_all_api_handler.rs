use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::component_model::ComponentModel;
use crate::responses::ApiResponse;
use axum::extract::State;
use axum::Json;
use std::sync::Arc;

pub async fn component_all_api_handler(
    state: State<Arc<AvoRedState>>,
) -> Result<Json<ApiResponse<Vec<ComponentModel>>>> {
    println!("->> {:<12} - component_all_api_handler", "HANDLER");
    let all_components = state.component_service.all(&state.db).await?;
    let response = ApiResponse {
        status: true,
        data: all_components,
    };
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use crate::api::rest_api_routes::tests::{get_auth_token, get_axum_app, send_get_request};
    use crate::error::{Error, Result};
    use crate::models::component_model::ComponentModel;
    use crate::repositories::into_iter_objects;
    use crate::responses::ApiResponse;
    use axum::http::StatusCode;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_component_all_api_handler() -> Result<()> {
        let (app, state) = get_axum_app().await.unwrap();
        let token = get_auth_token(state.clone())?;

        let sql = "
            CREATE components:content_id_1 CONTENT {
                name: 'unittest name 1',
                identifier: 'unittest identifier 1',
                element_type: 'TEXT',
                element_type: '[]',
                created_at: time::now(),
                updated_at: time::now(),
            }
        ";

        let (ds, ses) = &state.db;

        let query_responses = ds.execute(sql, ses, None).await?;
        let result_object_option = into_iter_objects(query_responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let component_model: ComponentModel = result_object?.try_into()?;

        let mut all_components: Vec<ComponentModel> = vec![];
        all_components.push(component_model);

        let dummy_res = ApiResponse {
            status: true,
            data: all_components,
        };

        let response = app
            .oneshot(send_get_request("/api/component-all", token))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let res_b = response.into_body();
        let body = axum::body::to_bytes(res_b, usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(&dummy_res));
        println!("Component_ALL: {:?}", json!(&dummy_res));
        Ok(())
    }
}
