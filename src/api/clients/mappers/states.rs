use crate::api::clients::clients_dbo::ClientDboState;
use crate::api::shared::daos::dbos::EntityDBO;
use crate::core::clients::data::{ClientActif, ClientStates};
use crate::core::shared::data::Entity;
use crate::models::clients::shared::ClientData;
impl From<ClientDboState> for ClientStates {
    fn from(value: ClientDboState) -> Self {
        match value {
            ClientDboState::ClientDbo { kind, first_name, last_name, birth_date } => ClientStates::Client(ClientActif {
                kind,
                data: ClientData {
                    first_name,
                    last_name,
                    birth_date,
                },
            })
        }
    }
}


impl From<Entity<ClientStates, String>> for EntityDBO<ClientDboState, String> {
    fn from(value: Entity<ClientStates, String>) -> Self {
        EntityDBO {
            id_mongo: None,
            version: value.version,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
        }
    }
}

impl From<ClientStates> for ClientDboState {
    fn from(value: ClientStates) -> Self {
        match value {
            ClientStates::Client(data) => {
                ClientDboState::ClientDbo {
                    kind: data.kind,
                    first_name: data.data.first_name,
                    last_name: data.data.last_name,
                    birth_date: data.data.birth_date,
                }
            }
        }
    }
}

impl From<EntityDBO<ClientDboState, String>> for Entity<ClientStates, String> {
    fn from(value: EntityDBO<ClientDboState, String>) -> Self {
        Self {
            entity_id: value.entity_id,
            data: value.data.into(),
            version: value.version,
        }
    }
}