use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
};
use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::api::rest_api::handlers::component::request::update_component_request::UpdateComponentRequest;
use crate::models::component_model::{ComponentModel, UpdatableComponentModel};
use crate::models::field_model::UpdatableFieldModel;
use crate::models::token_claim_model::LoggedInUser;

pub async fn update_component_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(component_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateComponentRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_component_api_handler", "HANDLER");


    let updateable_component_model = UpdatableComponentModel {
        id: component_id,
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email.clone(),
    };
    let mut updated_component_model = state
        .component_service
        .update_component(&state.db, updateable_component_model)
        .await?;

    for payload_field in payload.fields {

        println!("Payload Fields: {payload_field:?}");
        //@todo check for field ID and if not exist then create field and attached field
        let updatable_field = UpdatableFieldModel {
            id: payload_field.id,
            name: payload_field.name,
            identifier: payload_field.identifier,
            field_type: payload_field.field_type,
            logged_in_username: logged_in_user.email.clone(),
        };
        let updated_field = state
            .field_service
            .update_field(&state.db, updatable_field)
            .await?;

        updated_component_model.fields.push(updated_field);
        //
        // // println!("Created component {:?}", created_component.clone());
        // // println!("Created Field {:?}", created_field.clone());
        //
        // state
        //     .component_service
        //     .attach_component_with_field(
        //         &state.db,
        //         created_component.clone(),
        //         created_field.clone(),
        //         "admin@admin.com".to_string(),
        //     )
        //     .await?;
        // // println!("ATTACHED: {:?}", created_field.clone());
        //
        // created_component.fields.push(created_field);
    }


    let response = UpdatedComponentResponse {
        status: true,
        component_model: updated_component_model
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct UpdatedComponentResponse {
    pub status: bool,
    pub component_model: ComponentModel
}