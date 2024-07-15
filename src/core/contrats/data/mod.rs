use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

use crate::models::contrats::shared::{ContractData, CurrencyValue, PendingAmend, Vehicle};
use crate::models::contrats::views::{BaseContractStateView, ContractApprovedView, ContractCreatedView, ContractPendingAmendStateView, ContractRejectedView, ContractUpdatedView, ContractViewEvent, ContractViewState};
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
            ContratStates::Pending(c) => ContractViewState::Pending(BaseContractStateView {
                data: c.data.clone(),
                premium: c.premium.clone(),
            }),
            ContratStates::PendingAmendment(c) => ContractViewState::PendingAmendment(ContractPendingAmendStateView {
                data: c.data.clone(),
                premium: c.premium.clone(),
                pending_changes: c.pending_change.clone(),
            }),
            ContratStates::Actif(c) => ContractViewState::Actif(BaseContractStateView {
                data: c.data.clone(),
                premium: c.premium.clone(),
            }),
            ContratStates::Inactif(c) => ContractViewState::Inactif(BaseContractStateView {
                data: c.data.clone(),
                premium: c.premium.clone(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContratStates {
    Pending(PendingContract),
    PendingAmendment(PendingAmendContract),
    Actif(ActifContract),
    Inactif(InactifContract),
}

impl ContratStates {
    pub fn reduce_state(&self, event: &ContratEvents) -> Option<ContratStates> {
        match self {
            ContratStates::Pending(c) => c.reduce_state(event),
            ContratStates::PendingAmendment(c) => c.reduce_state(event),
            ContratStates::Actif(c) => c.reduce_state(event),
            ContratStates::Inactif(c) => c.reduce_state(event),
        }
    }

    pub fn reduce_state_from_empty(event: &ContratEvents) -> Option<ContratStates> {
        match event {
            ContratEvents::Created(e) => Some(
                ContratStates::Pending(
                    PendingContract {
                        data: e.data.clone(),
                        premium: e.premium.clone(),
                    }
                )
            ),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PendingContract {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

impl PendingContract {
    pub fn reduce_state(&self, event: &ContratEvents) -> Option<ContratStates> {
        match event {
            ContratEvents::Updated(e) => Some(
                ContratStates::Pending(
                    PendingContract {
                        data: ContractData {
                            holder: self.data.holder.clone(),
                            product: e.product.clone(),
                            formula: e.formula.clone(),
                            vehicle: e.vehicle.clone(),
                        },
                        premium: e.premium.clone(),
                    })),
            ContratEvents::Approved(_) => Some(
                ContratStates::Actif(
                    ActifContract {
                        data: self.data.clone(),
                        premium: self.premium.clone(),
                    }
                )
            ),
            ContratEvents::Rejected(_) => Some(
                ContratStates::Inactif(
                    InactifContract {
                        data: self.data.clone(),
                        premium: self.premium.clone(),
                    }
                )
            ),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PendingAmendContract {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
    pub pending_change: PendingAmend,
}

impl PendingAmendContract {
    pub fn reduce_state(&self, event: &ContratEvents) -> Option<ContratStates> {
        match event {
            ContratEvents::Approved(_) => Some(
                ContratStates::Actif(
                    ActifContract {
                        data: ContractData {
                            holder: self.data.holder.clone(),
                            product: self.pending_change.product.clone(),
                            formula: self.pending_change.formula.clone(),
                            vehicle: self.pending_change.vehicle.clone()
                        },
                        premium: self.pending_change.premium.clone(),
                    }
                )
            ),
            ContratEvents::Rejected(_) => Some(
                ContratStates::Actif(
                    ActifContract {
                        data: self.data.clone(),
                        premium: self.premium.clone(),
                    }
                )
            ),
            _ => None
        }
    }
}



#[derive(Serialize, Deserialize, Clone)]
pub struct ActifContract {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

impl ActifContract {
    pub fn reduce_state(&self, event: &ContratEvents) -> Option<ContratStates> {
        match event {
            ContratEvents::Updated(e) => Some(
                ContratStates::PendingAmendment(
                    PendingAmendContract {
                        data: self.data.clone(),
                        premium: self.premium.clone(),
                        pending_change: PendingAmend {
                            product: e.product.clone(),
                            formula: e.formula.clone(),
                            vehicle: e.vehicle.clone(),
                            premium: e.premium.clone()
                        }
                    })),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InactifContract {
    #[serde(flatten)]
    pub data: ContractData,
    pub premium: CurrencyValue,
}

impl InactifContract {
    pub fn reduce_state(&self, _event: &ContratEvents) -> Option<ContratStates> {
        None
    }
}


impl CanBeView<ContractViewEvent> for ContratEvents {
    fn to_view(&self) -> ContractViewEvent {
        match self {
            ContratEvents::Created(c) => ContractViewEvent::Created(ContractCreatedView { data: c.data.clone(), premium: c.premium.clone() }),
            ContratEvents::Updated(c) => ContractViewEvent::Updated(
                ContractUpdatedView {
                    formula: c.formula.clone(),
                    product: c.product.clone(),
                    vehicle: c.vehicle.clone(),
                    premium: c.premium.clone(),
                }),
            ContratEvents::Approved(c) => ContractViewEvent::Approved(ContractApprovedView {
                approved_by: c.approved_by.clone()
            }),
            ContratEvents::Rejected(c) => ContractViewEvent::Rejected(ContractRejectedView {
                rejected_by: c.reject_by.clone(),
                comment: c.comment.clone(),
            }),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub enum ContratEvents {
    Created(CreatedEvent),
    Approved(ApprovedEvent),
    Rejected(RejectEvent),
    Updated(UpdatedEvent),
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
pub struct ApprovedEvent {
    #[serde(rename = "approvedBy")]
    pub approved_by: UserInfo,
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RejectEvent {
    #[serde(rename = "rejectBy")]
    pub reject_by: UserInfo,
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub uid: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub email: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    pub product: String,
    pub formula: String,
    pub vehicle: Vehicle,
    pub premium: CurrencyValue,
}