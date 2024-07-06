use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use crate::models::clients::shared::ClientData;
use crate::models::clients::views::{ClientUpdatedView, ClientView, ClientViewActif, ClientViewEvent, ClientViewState};
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum ClientStates {
    #[serde(rename = "actif")]
    ClientActif(ClientActif)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientActif {
    #[serde(rename = "_kind")]
    pub kind: String,
    #[serde(flatten)]
    pub data: ClientData
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
                ClientViewState::Client (
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
            ClientEvents::Created(c) => ClientViewEvent::Created (ClientView { data : c.data.clone()}),
            ClientEvents::Updated(u) => ClientViewEvent::Updated(ClientUpdatedView { name: u.name.clone() })
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "eventType")]
pub enum ClientEvents {
    #[serde(rename = "created")]
    Created(CreatedEvent),
    #[serde(rename = "updated")]
    Updated(UpdatedEvent),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "typecustom", rename = "pouet")]
pub struct CreatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ClientData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    pub name: String,
}