use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

use crate::models::contrats::shared::ContractData;
use crate::models::contrats::views::{ContractUpdatedView, ContractView, ContractViewEvent};
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};

impl CanGetTypee for ContratStates {
    fn get_type(&self) -> String {
        "org:example:insurance:contract".to_string()
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


impl CanBeView<ContractViewEvent> for ContratEvents {
    fn to_view(&self) -> ContractViewEvent {
        match self {
            ContratEvents::Created(c) => ContractViewEvent::Created(ContractView {data: c.data.clone()}),
            ContratEvents::Updated(c) => ContractViewEvent::Updated(ContractUpdatedView {data: c.data.clone()}),
        }
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