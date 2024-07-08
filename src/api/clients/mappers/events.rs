use crate::api::shared::daos::dbos::EventDBO;
use crate::api::clients::clients_dbo::{ClientCreatedDbo, ClientDboEvent, ClientUpdatedDbo};
use crate::core::shared::data::EntityEvent;
use crate::core::clients::data::{ClientEvents, CreatedEvent, UpdatedEvent};
use crate::models::clients::shared::ClientData;

impl From<ClientDboEvent> for ClientEvents {
    fn from(value: ClientDboEvent) -> Self {
        match value {
            ClientDboEvent::Created(event_dbo) =>
                ClientEvents::Created(CreatedEvent {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: ClientData {
                        first_name: event_dbo.data.first_name,
                        last_name: event_dbo.data.last_name,
                        birth_date: event_dbo.data.birth_date,
                    }

                }),
            ClientDboEvent::Updated(event_dbo) => ClientEvents::Updated(UpdatedEvent { by: event_dbo.by, at: event_dbo.at, data: event_dbo.data })
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
            ClientEvents::Created(
                CreatedEvent {
                    by,
                    at,
                    data
                }
            ) => ClientDboEvent::Created(ClientCreatedDbo { by, at, data }),
            ClientEvents::Updated(updated) => ClientDboEvent::Updated(ClientUpdatedDbo { by: updated.by, at: updated.at, data: updated.data })
        }
    }
}

