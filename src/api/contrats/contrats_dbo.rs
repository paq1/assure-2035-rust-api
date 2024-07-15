use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::contrats::shared::{ContractData, CurrencyValue, Vehicle};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContratDboState {
    ContratPendingDbo(ContractPendingDbo),
    ContratPendingAmendmentDbo(ContractPendingDbo),
    ContratActifDbo(ContractActifDbo),
    ContratInactifDbo(ContractInactifDbo),
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
pub struct ContractInactifDbo {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContratDboEvent {
    ContratCreatedDbo(CreatedDbo),
    ApprovedDbo(ApprovedDbo),
    RefusedDbo(RefusedDbo),
    Updated(ContratUpdatedDbo),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovedDbo {
    pub approved_by: UserInfoDbo,
    pub by: String,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefusedDbo {
    pub refused_by: UserInfoDbo,
    pub by: String,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoDbo {
    pub uid: String,
    pub display_name: String,
    pub email: String,
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
    pub product: String,
    pub formula: String,
    pub vehicle: Vehicle,
    pub premium: CurrencyValue,
}
