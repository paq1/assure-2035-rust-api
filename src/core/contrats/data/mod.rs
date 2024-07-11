use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use crate::models::contrats::shared::ContractData;

#[derive(Serialize, Deserialize, Clone)]
pub enum ContratStates {
    Contract(Contract)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Contract {
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContratEvents {
    Created(CreatedEvent),
    Updated(UpdatedEvent)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ContractData
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ContractData
}