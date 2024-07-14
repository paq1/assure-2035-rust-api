use crate::api::contrats::contrats_dbo::{ApprovedDbo, ContratDboEvent, ContratUpdatedDbo, CreatedDbo, RefusedDbo};
use crate::api::shared::daos::dbos::EventDBO;
use crate::core::contrats::data::{ApprovedEvent, ContratEvents, CreatedEvent, RefusedEvent, UpdatedEvent};
use crate::core::shared::data::EntityEvent;

pub mod user_info;

impl From<ContratDboEvent> for ContratEvents {
    fn from(value: ContratDboEvent) -> Self {
        match value {
            ContratDboEvent::ContratCreatedDbo(event_dbo) => ContratEvents::Created(
                CreatedEvent {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data,
                    premium: event_dbo.premium,
                }
            ),
            ContratDboEvent::Updated(event_dbo) =>
                ContratEvents::Updated(UpdatedEvent { by: event_dbo.by, at: event_dbo.at, data: event_dbo.data }),
            ContratDboEvent::ApprovedDbo(event_dbo) =>
                ContratEvents::Approved(ApprovedEvent {
                    approved_by: event_dbo.approved_by.into(),
                    by: event_dbo.by,
                    at: event_dbo.at,
                }),
            ContratDboEvent::RefusedDbo(event_dbo) =>
                ContratEvents::Refused(RefusedEvent {
                    refused_by: event_dbo.refused_by.into(),
                    by: event_dbo.by,
                    at: event_dbo.at,
                })
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
            ContratEvents::Created(event) => ContratDboEvent::ContratCreatedDbo(
                CreatedDbo {
                    by: event.by,
                    at: event.at,
                    data: event.data,
                    premium: event.premium,
                }
            ),
            ContratEvents::Updated(updated) => ContratDboEvent::Updated(
                ContratUpdatedDbo {
                    by: updated.by,
                    at: updated.at,
                    data: updated.data,
                }),
            ContratEvents::Approved(approved) => ContratDboEvent::ApprovedDbo(
                ApprovedDbo {
                    by: approved.by,
                    at: approved.at,
                    approved_by: approved.approved_by.into(),
                }),
            ContratEvents::Refused(refused) => ContratDboEvent::RefusedDbo(
                RefusedDbo {
                    by: refused.by,
                    at: refused.at,
                    refused_by: refused.refused_by.into(),
                }
            )
        }
    }
}

