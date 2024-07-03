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
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateClientCommand {
    #[schema(example = "input")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DeleteClientCommand;
