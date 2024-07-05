use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::clients::shared::ClientData;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ClientsCommands {
    Create (CreateClientCommand),
    Update (UpdateClientCommand),
    Delete (DeleteClientCommand)
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateClientCommand {
    #[serde(flatten)]
    pub data: ClientData
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateClientCommand {
    #[schema(example = "input")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DeleteClientCommand;
