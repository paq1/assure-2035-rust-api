use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::client_disable::ClientDisable;
use crate::core::clients::data::states::ClientStates;
use crate::models::clients::shared::ClientData;
use crate::models::clients::views::ClientViewActif;
use crate::models::shared::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct ClientActif {
    pub kind: String,
    pub data: ClientData,
}

impl ClientActif {
    pub fn reduce_state(&self, event: ClientEvents) -> Option<ClientStates> {
        match event {
            ClientEvents::Updated(updated) => Some(
                ClientStates::ClientActif(
                    ClientActif {
                        kind: self.kind.clone(),
                        data: updated.data.clone(),
                    }
                )
            ),
            ClientEvents::Disabled(disabled) => Some(
                ClientStates::ClientDisable(
                    ClientDisable {
                        kind: self.kind.clone(),
                        data: self.data.clone(),
                        reason: disabled.reason,
                    }
                )
            ),
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for ClientActif {
    fn get_type(&self) -> String {
        "org:example:insurance:client".to_string()
    }
}

impl From<ClientActif> for ClientViewActif {
    fn from(value: ClientActif) -> Self {
        ClientViewActif {
            data: value.data.clone()
        }
    }
}
