use serde_json::Value;

use crate::{api::proto::web_event::{WebEventRequest, WebEventResponse}, error::Result, models::web_event_model::{WebEventModel, WebEventPayloadField}};

/// Setting service 
pub struct WebEventService {}

impl WebEventService {
    /// Creates a new instance of `WebEventService`
    pub const fn new() -> Result<Self> {
        Ok(Self {  })
    }

    /// Sends a web event based on the provided request.
    pub async fn send_event(
        &self,
        request: WebEventRequest,
    ) -> Result<WebEventResponse> {
        println!("Processing event in WebEventService: {:?}", request);

        // Based on the souce of the request we load web_event_model from DB
        // based on the model fields we validate the payload
        // perform the workflow actions

        // think about what are way we setup action and action meta data
        let web_event_fields = WebEventPayloadField {
            key: "name".to_string(),
            validations: vec!["required".to_string()],
            action: crate::models::web_event_model::WebEventActionType::CreateRecord,
            action_metadata: None
        };

        let web_event_model = WebEventModel {
            id: request.id.clone(),
            fields: vec![web_event_fields],
        };


        let payload: Value = serde_json::from_str(&request.payload).unwrap();

        for field in &web_event_model.fields {
            let field_value = payload.get(&field.key).unwrap_or_default();

            println!("Field value: {field_value:?}");
        }

        println!("value: {payload:?}");


        // Here you can add logic to process the event, e.g., store it in a database, trigger other actions, etc.
        let response = WebEventResponse {
            success: true,
            message: format!("Received event"),
        };
        Ok(response)
    }
    
}