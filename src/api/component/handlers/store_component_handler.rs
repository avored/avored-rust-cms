use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
    models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use avored_better_query::AvoRedForm;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use validator::HasLen;
use crate::api::component::requests::store_component_request::StoreComponentRequest;
use crate::models::component_model::{CreatableComponent, FieldModel};
use crate::providers::avored_view_provider::translate;

pub async fn store_component_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession,
    AvoRedForm(payload): AvoRedForm<StoreComponentRequest>,
) -> Result<impl IntoResponse> {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    println!("Payload: {:?}", payload);

    let validation_error_list = payload.validate_errors(session.clone())?;
    if validation_error_list.errors().length() > 0 {
        return Ok(Redirect::to("/admin/create-role").into_response());
    }
    let mut fields = Vec::new();
    for payload_field in payload.fields {
        let field_model = FieldModel {
            name: payload_field.name,
            identifier: payload_field.identifier,
            field_type: payload_field.field_type
        };
        fields.push(field_model);
    }

    let creatable_component = CreatableComponent {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
        fields
    };

    let _created_component = state
        .component_service
        .create_component(&state.db, creatable_component)
        .await;
    session
        .insert("success_message", translate("success_created_component"))
        .expect("Could not store the success message into session.");

    Ok(Redirect::to("/admin/component").into_response())
}
