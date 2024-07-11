use crate::api::contrats::contrats_dbo::{ContratDboEvent, ContratUpdatedDbo, CreatedDbo};
use crate::api::shared::daos::dbos::EventDBO;
use crate::core::contrats::data::{ContratEvents, CreatedEvent, UpdatedEvent};
use crate::core::shared::data::EntityEvent;

impl From<ContratDboEvent> for ContratEvents {
    fn from(value: ContratDboEvent) -> Self {
        match value {
            ContratDboEvent::ContratCreatedDbo (event_dbo) => ContratEvents::Created(
                CreatedEvent {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data,
                    premieum: event_dbo.premieum,
                }
            ),
            ContratDboEvent::Updated(event_dbo) => ContratEvents::Updated(UpdatedEvent { by: event_dbo.by, at: event_dbo.at, data: event_dbo.data })
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
            ContratEvents::Created (event) => ContratDboEvent::ContratCreatedDbo (
                CreatedDbo {
                    by: event.by,
                    at: event.at,
                    data: event.data,
                    premieum: event.premieum
                }
            ),
            ContratEvents::Updated(updated) => ContratDboEvent::Updated(ContratUpdatedDbo { by: updated.by, at: updated.at, data: updated.data })
        }
    }
}

