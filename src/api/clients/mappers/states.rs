use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::clients::clients_dbo::ClientDboState;
use crate::core::shared::data::Entity;
use crate::core::clients::data::ClientStates;

impl From<ClientDboState> for ClientStates {
    fn from(value: ClientDboState) -> Self {
        match value {
            ClientDboState::ClientDbo { name } => ClientStates::Client {
                name: name.clone()
            }
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
            ClientStates::Client { name } => ClientDboState::ClientDbo { name: name.clone() }
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