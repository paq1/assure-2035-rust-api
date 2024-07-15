use crate::api::contrats::contrats_dbo::{ContractActifDbo, ContractInactifDbo, ContractPendingAmendmentDbo, ContractPendingDbo, ContratDboState};
use crate::api::shared::daos::dbos::EntityDBO;
use crate::core::contrats::data::{ActifContract, ContratStates, InactifContract, PendingAmendContract, PendingContract};
use crate::core::shared::data::Entity;
use crate::models::contrats::shared::PendingAmend;

impl From<ContratDboState> for ContratStates {
    fn from(value: ContratDboState) -> Self {
        match value {
            ContratDboState::ContratPendingDbo(dbo) => ContratStates::Pending(PendingContract {
                data: dbo.data,
                premium: dbo.premium,
            }),
            ContratDboState::ContratPendingAmendmentDbo(dbo) => ContratStates::PendingAmendment(PendingAmendContract {
                data: dbo.data,
                premium: dbo.premium,
                pending_change: dbo.pending_change,
            }),
            ContratDboState::ContratActifDbo(dbo) => ContratStates::Actif(ActifContract {
                data: dbo.data,
                premium: dbo.premium,
            }),
            ContratDboState::ContratInactifDbo(dbo) => ContratStates::Inactif(InactifContract {
                data: dbo.data,
                premium: dbo.premium,
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
            ContratStates::Pending(contract) => ContratDboState::ContratPendingDbo(
                ContractPendingDbo {
                    data: contract.data,
                    premium: contract.premium,
                }
            ),
            ContratStates::PendingAmendment(contract) => ContratDboState::ContratPendingAmendmentDbo(
                ContractPendingAmendmentDbo {
                    data: contract.data,
                    premium: contract.premium,
                    pending_change: contract.pending_change,
                }
            ),
            ContratStates::Actif(contract) => ContratDboState::ContratActifDbo(
                ContractActifDbo {
                    data: contract.data,
                    premium: contract.premium,
                }
            ),
            ContratStates::Inactif(contract) => ContratDboState::ContratInactifDbo(
                ContractInactifDbo {
                    data: contract.data,
                    premium: contract.premium,
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