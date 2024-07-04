use crate::api::contrats::contrats_dbo::{ContratDboEvent, ContratUpdatedDbo};
use crate::api::shared::daos::dbos::EventDBO;
use crate::core::contrats::data::{ContratEvents, UpdatedEvent};
use crate::core::shared::data::EntityEvent;

impl From<ContratDboEvent> for ContratEvents {
    fn from(value: ContratDboEvent) -> Self {
        match value {
            ContratDboEvent::ContratCreatedDbo { by, at, name } => ContratEvents::Created { by, at, name },
            ContratDboEvent::Updated(event_dbo) => ContratEvents::Updated(UpdatedEvent { by: event_dbo.by, at: event_dbo.at, name: event_dbo.name })
        }
    }
}

impl From<EventDBO<ContratDboEvent, String>> for EntityEvent<ContratEvents, String> {
    fn from(value: EventDBO<ContratDboEvent, String>) -> Self {
        EntityEvent {
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone(),
        }
    }
}


impl From<EntityEvent<ContratEvents, String>> for EventDBO<ContratDboEvent, String> {
    fn from(value: EntityEvent<ContratEvents, String>) -> Self {
        EventDBO {
            id_mongo: None,
            version: None,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone(),
        }
    }
}

impl From<ContratEvents> for ContratDboEvent {
    fn from(value: ContratEvents) -> Self {
        match value {
            ContratEvents::Created { by, at, name } => ContratDboEvent::ContratCreatedDbo { by, at, name },
            ContratEvents::Updated(updated) => ContratDboEvent::Updated(ContratUpdatedDbo { by: updated.by, at: updated.at, name: updated.name })
        }
    }
}

