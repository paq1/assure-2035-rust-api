use chrono::{DateTime, Utc};

use crate::models::clients::shared::{ClientData, DisableReason};
use crate::models::clients::views::{ClientDisabledView, ClientUpdatedView, ClientView, ClientViewActif, ClientViewDisable, ClientViewEvent, ClientViewState};
use crate::models::shared::errors::ResultErr;
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};

#[derive(Clone, Debug)]
pub enum ClientStates {
    ClientActif(ClientActif),
    ClientDisable(ClientDisable),
}

#[derive(Clone, Debug)]
pub struct ClientActif {
    pub kind: String,
    pub data: ClientData,
}

#[derive(Clone, Debug)]
pub struct ClientDisable {
    pub kind: String,
    pub data: ClientData,
    pub reason: DisableReason
}

impl ClientStates {
    pub fn data(&self) -> ResultErr<ClientData> {
        match self {
            ClientStates::ClientActif(client_data) => Ok(client_data.data.clone()),
            ClientStates::ClientDisable(state) => Ok(state.data.clone()),
        }
    }
}

impl CanGetTypee for ClientStates {
    fn get_type(&self) -> String { // fixme trouver mieux que ce truc
        match self {
            ClientStates::ClientActif(_c) => "org:example:insurance:client".to_string(),
            ClientStates::ClientDisable(_c) => "org:example:insurance:client".to_string(),
        }
    }
}

impl CanBeView<ClientViewState> for ClientStates {
    fn to_view(&self) -> ClientViewState {
        match self {
            ClientStates::ClientActif(state) =>
                ClientViewState::Client(
                    ClientViewActif {
                        data: ClientData {
                            first_name: state.data.first_name.clone(),
                            last_name: state.data.last_name.clone(),
                            birth_date: state.data.birth_date,
                        }
                    }
                ),
            ClientStates::ClientDisable(state) =>
                ClientViewState::ClientDisable(
                    ClientViewDisable {
                        data: ClientData {
                            first_name: state.data.first_name.clone(),
                            last_name: state.data.last_name.clone(),
                            birth_date: state.data.birth_date,
                        }
                    }
                )
        }
    }
}

impl CanBeView<ClientViewEvent> for ClientEvents {
    fn to_view(&self) -> ClientViewEvent {
        match self {
            ClientEvents::Created(c) => ClientViewEvent::Created(ClientView { data: c.data.clone() }),
            ClientEvents::Updated(u) => ClientViewEvent::Updated(ClientUpdatedView { data: u.data.clone() }),
            ClientEvents::Disabled(u) => ClientViewEvent::Disabled(ClientDisabledView { data: u.data.clone() }),
        }
    }
}


#[derive(Clone)]
pub enum ClientEvents {
    Created(CreatedEvent),
    Updated(UpdatedEvent),
    Disabled(DisabledEvent),
}

#[derive(Clone)]
pub struct CreatedEvent {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: ClientData,
}

#[derive(Clone)]
pub struct UpdatedEvent {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: ClientData,
}

#[derive(Clone)]
pub struct DisabledEvent {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: ClientData,
    pub reason: DisableReason
}
