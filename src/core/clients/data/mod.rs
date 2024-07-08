use chrono::{DateTime, Utc};

use crate::models::clients::shared::ClientData;
use crate::models::clients::views::{ClientUpdatedView, ClientView, ClientViewActif, ClientViewEvent, ClientViewState};
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};

#[derive(Clone, Debug)]
pub enum ClientStates {
    ClientActif(ClientActif)
}

#[derive(Clone, Debug)]
pub struct ClientActif {
    pub kind: String,
    pub data: ClientData,
}

impl ClientStates {
    pub fn data(&self) -> ClientData {
        match self {
            ClientStates::ClientActif(client_data) => client_data.data.clone()
        }
    }
}

impl CanGetTypee for ClientStates {
    fn get_type(&self) -> String {
        match self {
            ClientStates::ClientActif(_c) => "org:example:insurance:client".to_string()
        }
    }
}

impl CanBeView<ClientViewState> for ClientStates {
    fn to_view(&self) -> ClientViewState {
        match self {
            ClientStates::ClientActif(d) =>
                ClientViewState::Client(
                    ClientViewActif {
                        data: ClientData {
                            first_name: d.data.first_name.clone(),
                            last_name: d.data.last_name.clone(),
                            birth_date: d.data.birth_date,
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
            ClientEvents::Updated(u) => ClientViewEvent::Updated(ClientUpdatedView { data: u.data.clone() })
        }
    }
}


#[derive(Clone)]
pub enum ClientEvents {
    Created(CreatedEvent),
    Updated(UpdatedEvent),
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