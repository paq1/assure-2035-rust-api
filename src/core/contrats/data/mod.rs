use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

use crate::models::contrats::shared::ContractData;
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};

impl CanGetTypee for ContratStates {
    fn get_type(&self) -> String {
        "org:example:insurance:client".to_string()
    }
}

impl CanBeView<ContratStates> for ContratStates {
    fn to_view(&self) -> ContratStates {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContratStates {
    Contract(Contract)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Contract {
    #[serde(flatten)]
    pub data: ContractData
}


impl CanBeView<ContratEvents> for ContratEvents {
    fn to_view(&self) -> ContratEvents {
        return self.clone()
    }
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