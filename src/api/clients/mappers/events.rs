use crate::api::shared::daos::dbos::EventDBO;
use crate::api::clients::clients_dbo::{ClientCreatedDbo, ClientDboEvent, ClientUpdatedDbo};
use crate::core::shared::data::EntityEvent;
use crate::core::clients::data::{ClientEvents, UpdatedEvent};

impl From<ClientDboEvent> for ClientEvents {
    fn from(value: ClientDboEvent) -> Self {
        match value {
            ClientDboEvent::Created(event_dbo) => ClientEvents::Created { by: event_dbo.by, at: event_dbo.at, name: event_dbo.name },
            ClientDboEvent::Updated(event_dbo) => ClientEvents::Updated(UpdatedEvent { by: event_dbo.by, at: event_dbo.at, name: event_dbo.name })
        }
    }
}

impl From<EventDBO<ClientDboEvent, String>> for EntityEvent<ClientEvents, String> {
    fn from(value: EventDBO<ClientDboEvent, String>) -> Self {
        EntityEvent {
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone(),
        }
    }
}


impl From<EntityEvent<ClientEvents, String>> for EventDBO<ClientDboEvent, String> {
    fn from(value: EntityEvent<ClientEvents, String>) -> Self {
        EventDBO {
            id_mongo: None,
            version: None,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone(),
        }
    }
}

impl From<ClientEvents> for ClientDboEvent {
    fn from(value: ClientEvents) -> Self {
        match value {
            ClientEvents::Created { by, at, name } => ClientDboEvent::Created( ClientCreatedDbo { by, at, name }),
            ClientEvents::Updated(updated) => ClientDboEvent::Updated(ClientUpdatedDbo { by: updated.by, at: updated.at, name: updated.name })
        }
    }
}

