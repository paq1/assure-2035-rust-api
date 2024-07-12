use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::contrats::shared::{ContractData, CurrencyValue};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContratDboState {
    ContratPendingDbo(ContractPendingDbo),
    ContratActifDbo(ContractActifDbo),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractPendingDbo {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractActifDbo {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContratDboEvent {
    ContratCreatedDbo(CreatedDbo),
    ApprovedDbo(ApprovedDbo),
    Updated(ContratUpdatedDbo),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContratUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ContractData,
}
