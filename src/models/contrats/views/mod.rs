use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::contrats::shared::{ContractData, CurrencyValue};

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ContratView {
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum ContractViewState {
    #[serde(rename = "pending-subscription")]
    Pending(BaseContractStateView)
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum ContractViewEvent {
    #[serde(rename = "created")]
    Created(ContractCreatedView),
    #[serde(rename = "updated")]
    Updated(ContractUpdatedView),
    #[serde(rename = "approved")]
    Approved(ContractApprovedView)
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractCreatedView {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}


#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct BaseContractStateView {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractUpdatedView {
    #[serde(flatten)]
    pub data: ContractData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractApprovedView {}