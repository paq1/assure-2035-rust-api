use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::clients::shared::{ClientData, DisableReason};

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ClientsCommands {
    Create (CreateClientCommand),
    Update (UpdateClientCommand),
    Disable(DisableClientCommand)
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateClientCommand {
    #[serde(flatten)]
    pub data: ClientData
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateClientCommand {
    #[serde(flatten)]
    pub data: ClientData
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DisableClientCommand {
    pub reason: DisableReason
}
