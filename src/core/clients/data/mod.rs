use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use crate::models::clients::shared::ClientData;
use crate::models::clients::views::{ClientUpdatedView, ClientView, ClientViewEvent};
use crate::models::shared::jsonapi::CanBeView;

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientStates {
    Client (ClientData)
}

impl ClientStates {
    pub fn data(&self) -> ClientData {
        match self {
            ClientStates::Client(client_data) => client_data.clone()
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