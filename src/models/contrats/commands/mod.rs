use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::contrats::shared::ContractData;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ContratsCommands {
    Create (CreateContratCommand),
    Update (UpdateContratCommand),
    Delete (DeleteContratCommand)
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateContratCommand {
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateContratCommand {
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DeleteContratCommand;
