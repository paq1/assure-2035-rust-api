use crate::api::contrats::contrats_dbo::{ContractActifDbo, ContractPendingDbo, ContratDboState};
use crate::api::shared::daos::dbos::EntityDBO;
use crate::core::contrats::data::{PendingContract, ContratStates, ActifContract};
use crate::core::shared::data::Entity;

impl From<ContratDboState> for ContratStates {
    fn from(value: ContratDboState) -> Self {
        match value {
            ContratDboState::ContratPendingDbo(dbo ) => ContratStates::Pending(PendingContract {
                data: dbo.data,
                premium: dbo.premium
            }),
            ContratDboState::ContratActifDbo(dbo) => ContratStates::Actif(ActifContract {
                data: dbo.data,
                premium: dbo.premium
            })
        }
    }
}


impl From<Entity<ContratStates, String>> for EntityDBO<ContratDboState, String> {
    fn from(value: Entity<ContratStates, String>) -> Self {
        EntityDBO {
            id_mongo: None,
            version: value.version,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
        }
    }
}

impl From<ContratStates> for ContratDboState {
    fn from(value: ContratStates) -> Self {
        match value {
            ContratStates::Pending(contract ) => ContratDboState::ContratPendingDbo(
                ContractPendingDbo {
                    data: contract.data,
                    premium: contract.premium
                }
            ),
            ContratStates::Actif(contract) => ContratDboState::ContratActifDbo(
                ContractActifDbo {
                    data: contract.data,
                    premium: contract.premium
                }
            )
        }
    }
}

impl From<EntityDBO<ContratDboState, String>> for Entity<ContratStates, String> {
    fn from(value: EntityDBO<ContratDboState, String>) -> Self {
        Self {
            entity_id: value.entity_id,
            data: value.data.into(),
            version: value.version,
        }
    }
}