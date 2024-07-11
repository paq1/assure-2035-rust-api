use crate::api::clients::clients_dbo::ClientDboState;
use crate::api::shared::daos::dbos::EntityDBO;
use crate::core::clients::data::states::client_actif::ClientActif;
use crate::core::clients::data::states::client_disable::ClientDisable;
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::data::Entity;

impl From<ClientDboState> for ClientStates {
    fn from(value: ClientDboState) -> Self {
        match value {
            ClientDboState::ClientDbo { kind, data } =>
                ClientStates::ClientActif(
                    ClientActif {
                        kind,
                        data,
                    }
                ),
            ClientDboState::ClientDisableDbo { kind, data, reason } =>
                ClientStates::ClientDisable(
                    ClientDisable {
                        kind,
                        data,
                        reason,
                    }
                )
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
            ClientStates::ClientActif(data) => {
                ClientDboState::ClientDbo {
                    kind: data.kind,
                    data: data.data,
                }
            }
            ClientStates::ClientDisable(data) => {
                ClientDboState::ClientDisableDbo {
                    kind: data.kind,
                    data: data.data,
                    reason: data.reason,
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