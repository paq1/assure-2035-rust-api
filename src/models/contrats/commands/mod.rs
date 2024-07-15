use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::contrats::shared::{ContractData, Vehicle};

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum ContratsCommands {
    Create(CreateContratCommand),
    Approve(ApproveContractCommand),
    Reject(RejectContractCommand),
    Terminate(TerminateContractCommand),
    Update(UpdateContratCommand),
    Delete(DeleteContratCommand),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct CreateContratCommand {
    #[serde(flatten)]
    pub data: ContractData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct UpdateContratCommand {
    pub product: String,
    pub formula: String,
    pub vehicle: Vehicle,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ApproveContractCommand {}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RejectContractCommand {
    pub comment: String
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct TerminateContractCommand {
    pub reason: String // fixme mettre un enum
}


#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct DeleteContratCommand {}
