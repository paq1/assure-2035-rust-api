use chrono::{DateTime, Utc};

use crate::models::clients::shared::{ClientData, DisableReason};
use crate::models::clients::views::{ClientDisabledView, ClientUpdatedView, ClientView, ClientViewEvent};
use crate::models::shared::jsonapi::CanBeView;

impl CanBeView<ClientViewEvent> for ClientEvents {
    fn to_view(&self) -> ClientViewEvent {
        match self {
            ClientEvents::Created(c) => ClientViewEvent::Created(ClientView { data: c.data.clone() }),
            ClientEvents::Updated(u) => ClientViewEvent::Updated(ClientUpdatedView { data: u.data.clone() }),
            ClientEvents::Disabled(disabled) => ClientViewEvent::Disabled(ClientDisabledView { reason: disabled.reason.clone() }),
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
    pub reason: DisableReason,
}
