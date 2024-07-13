use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::models::clients::shared::{ClientData, DisableReason};
use crate::models::clients::views::ClientViewDisable;
use crate::models::shared::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct ClientDisable {
    pub kind: String,
    pub data: ClientData,
    pub reason: DisableReason,
}

impl ClientDisable {
    pub fn reduce_state(&self, event: ClientEvents) -> Option<ClientStates> {
        match event {
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for ClientDisable {
    fn get_type(&self) -> String {
        "org:example:insurance:client".to_string()
    }
}

impl From<ClientDisable> for ClientViewDisable {
    fn from(value: ClientDisable) -> Self {
        ClientViewDisable {
            data: value.data
        }
    }
}
