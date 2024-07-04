use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ClientsCommands {
    Create (CreateClientCommand),
    Update (UpdateClientCommand),
    Delete (DeleteClientCommand)
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateClientCommand {
    #[schema(example = "input")]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateClientCommand {
    #[schema(example = "input")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DeleteClientCommand;
