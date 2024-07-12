use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

use crate::models::contrats::shared::{ContractData, CurrencyValue};
use crate::models::contrats::views::{ContractUpdatedView, ContractView, ContractViewEvent, ContractViewState};
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};

pub mod shared;

impl CanGetTypee for ContratStates {
    fn get_type(&self) -> String {
        "org:example:insurance:contract".to_string()
    }
}

impl CanBeView<ContractViewState> for ContratStates {
    fn to_view(&self) -> ContractViewState {
        match self {
            ContratStates::Contract(c) => ContractViewState::Contract(ContractView {
                data: c.data.clone(), premium: c.premium.clone()
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContratStates {
    Contract(Contract)
}

impl ContratStates {

    pub fn reduce_state(&self, event: &ContratEvents) -> Option<ContratStates> {
        match self {
            ContratStates::Contract(c) => c.reduce_state(event)
        }
    }

    pub fn reduce_state_from_empty(event: &ContratEvents) -> Option<ContratStates> {
        match event {
            ContratEvents::Created(e) => Some(
                ContratStates::Contract(
                    Contract {
                        data: e.data.clone(),
                        premium: e.premium.clone()
                    }
                )
            ),
            _ => None
        }
    }

}

#[derive(Serialize, Deserialize, Clone)]
pub struct Contract {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

impl Contract {
    pub fn reduce_state(&self, event: &ContratEvents) -> Option<ContratStates> {
        match event {
            ContratEvents::Updated (e) => Some(
                ContratStates::Contract (
                    Contract {
                        data: e.data.clone(),
                        premium: self.premium.clone()
                    })),
            _ => None
        }
    }
}


impl CanBeView<ContractViewEvent> for ContratEvents {
    fn to_view(&self) -> ContractViewEvent {
        match self {
            ContratEvents::Created(c) => ContractViewEvent::Created(ContractView {data: c.data.clone(), premium: c.premium.clone()}),
            ContratEvents::Updated(c) => ContractViewEvent::Updated(ContractUpdatedView { data: c.data.clone() }),
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
    pub data: ContractData,
    pub premium: CurrencyValue,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ContractData
}