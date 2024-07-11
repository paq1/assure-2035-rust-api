use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::contrats::shared::ContractData;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ContratView {
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum ContractViewState {
    Contract(ContractView)
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum ContractViewEvent {
    #[serde(rename = "created")]
    Created(ContractView),
    #[serde(rename = "updated")]
    Updated(ContractUpdatedView),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractView {
    #[serde(flatten)]
    pub data: ContractData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ContractUpdatedView {
    #[serde(flatten)]
    pub data: ContractData,
}