use std::sync::Arc;

use crate::api::component::requests::store_component_request::StoreComponentRequest;
use crate::models::component_model::CreatableComponent;
use crate::models::field_model::CreatableFieldModel;
use crate::providers::avored_view_provider::translate;
use crate::{
    avored_state::AvoRedState, error::Result, models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use avored_better_query::AvoRedForm;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use validator::HasLen;

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
        return Ok(Redirect::to("/admin/create-component").into_response());
    }
    // let mut fields = Vec::new();
    // for payload_field in payload.fields {
    //     let field_model = FieldModel {
    //         name: payload_field.name,
    //         identifier: payload_field.identifier,
    //         field_type: payload_field.field_type
    //     };
    //     fields.push(field_model);
    // }

    let creatable_component = CreatableComponent {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email.clone(),
    };

    let created_component = state
        .component_service
        .create_component(&state.db, creatable_component)
        .await?;

    for payload_field in payload.fields {
        let creatable_field = CreatableFieldModel {
            name: payload_field.name,
            identifier: payload_field.identifier,
            logged_in_username: logged_in_user.email.clone(),
        };

        let created_field = state
            .field_service
            .create_field(&state.db, creatable_field)
            .await?;

        println!("Created component {:?}", created_component.clone());
        println!("Created Field {:?}", created_field.clone());

        let created_field = state
            .component_service
            .attach_component_with_field(
                &state.db,
                created_component.clone(),
                created_field,
                logged_in_user.email.clone(),
            )
            .await?;

        println!("ATTACHED: {:?}", created_field);
    }

    session
        .insert("success_message", translate("success_created_component"))
        .expect("Could not store the success message into session.");

    Ok(Redirect::to("/admin/component").into_response())
}
