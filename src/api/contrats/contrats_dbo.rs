use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::contrats::shared::ContractData;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContratDboState {
    ContratDbo(ContractDbo)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractDbo {
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContratDboEvent {
    ContratCreatedDbo(CreatedDbo),
    Updated(ContratUpdatedDbo)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContratUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ContractData
}
