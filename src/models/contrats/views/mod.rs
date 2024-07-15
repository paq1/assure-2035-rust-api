use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::core::contrats::data::UserInfo;
use crate::models::contrats::shared::{ContractData, CurrencyValue, PendingAmend, Vehicle};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum ContractViewState {
    #[serde(rename = "pending-subscription")]
    Pending(BaseContractStateView),
    #[serde(rename = "pending-amendment")]
    PendingAmendment(ContractPendingAmendStateView),
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
    #[serde(rename = "amended")]
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
pub struct ContractPendingAmendStateView {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
    #[serde(rename = "pendingChanges")]
    pub pending_changes: PendingAmend,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractUpdatedView {
    pub product: String,
    pub formula: String,
    pub vehicle: Vehicle,
    pub premium: CurrencyValue,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractApprovedView {
    #[serde(rename = "approvedBy")]
    pub approved_by: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractRefusedView {
    #[serde(rename = "refusedBy")]
    pub refused_by: UserInfo,
}