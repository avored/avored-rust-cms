use serde::{Deserialize, Serialize};


/// Represents an visitor log model in the system.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct WebEventModel {

    /// The unique identifier for the visitor log.
    pub id: String,

    /// the fields associated with web event model
    pub fields: Vec<WebEventPayloadField>
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreateActionMetadata {
    pub table_name: String,
}


/// Represents the type of action that needs to be performed on web event.
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub enum WebEventActionType {
    #[default]
    DoNothing,

    CreateRecord
}


/// Represents the type of action metadata needed to perform action.
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct WebEventActionMetaDataType {
    pub create_action_metadata: Option<CreateActionMetadata>,
}


/// Represents an visitor log model in the system.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct WebEventPayloadField {
    pub key: String,
    pub validations: Vec<String>,
    pub action: WebEventActionType,
    pub action_metadata: Option<WebEventActionMetaDataType>,
}