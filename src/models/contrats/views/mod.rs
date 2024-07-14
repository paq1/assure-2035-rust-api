use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::contrats::shared::{ContractData, CurrencyValue};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum ContractViewState {
    #[serde(rename = "pending-subscription")]
    Pending(BaseContractStateView),
    #[serde(rename = "actif")]
    Actif(BaseContractStateView),
    #[serde(rename = "inactif")]
    Inactif(BaseContractStateView),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum ContractViewEvent {
    #[serde(rename = "created")]
    Created(ContractCreatedView),
    #[serde(rename = "updated")]
    Updated(ContractUpdatedView),
    #[serde(rename = "approved")]
    Approved(ContractApprovedView),
    #[serde(rename = "refused")]
    Refused(ContractRefusedView),
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
pub struct ContractApprovedView;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractRefusedView;