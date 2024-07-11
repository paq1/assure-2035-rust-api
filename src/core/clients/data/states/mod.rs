use crate::core::clients::data::ClientEvents;
use crate::core::clients::data::ClientEvents::Created;
use crate::core::clients::data::states::client_actif::ClientActif;
use crate::core::clients::data::states::client_disable::ClientDisable;
use crate::models::clients::shared::ClientData;
use crate::models::clients::views::ClientViewState;
use crate::models::shared::errors::ResultErr;
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};

pub mod client_actif;
pub mod client_disable;

#[derive(Clone, Debug)]
pub enum ClientStates {
    ClientActif(ClientActif),
    ClientDisable(ClientDisable),
}

impl ClientStates {
    pub fn reduce_state(&self, event: ClientEvents) -> Option<ClientStates> {
        match self {
            ClientStates::ClientActif(c) => c.reduce_state(event),
            ClientStates::ClientDisable(c) => c.reduce_state(event),
        }
    }

    pub fn reduce_state_from_empty(event: ClientEvents) -> Option<ClientStates> {
        match event {
            Created(data) =>
                Some(
                    ClientStates::ClientActif(
                        ClientActif {
                            kind: "org:example:insurance:client".to_string(),
                            data: data.data,
                        }
                    )
                ),
            _ => None
        }
    }
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
    fn get_type(&self) -> String {
        match self {
            ClientStates::ClientActif(state) => state.get_type(),
            ClientStates::ClientDisable(state) => state.get_type(),
        }
    }
}

impl CanBeView<ClientViewState> for ClientStates {
    fn to_view(&self) -> ClientViewState {
        match self {
            ClientStates::ClientActif(state) =>
                ClientViewState::Client(state.clone().into()),
            ClientStates::ClientDisable(state) =>
                ClientViewState::ClientDisable(state.clone().into())
        }
    }
}