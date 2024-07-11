use crate::api::clients::clients_dbo::{ClientCreatedDbo, ClientDboEvent, ClientDisabledDbo, ClientUpdatedDbo};
use crate::api::shared::daos::dbos::EventDBO;
use crate::core::clients::data::{ClientEvents, CreatedEvent, DisabledEvent, UpdatedEvent};
use crate::core::shared::data::EntityEvent;
use crate::models::clients::shared::ClientData;

impl From<ClientDboEvent> for ClientEvents {
    fn from(value: ClientDboEvent) -> Self {
        match value {
            ClientDboEvent::Created(event_dbo) =>
                ClientEvents::Created(CreatedEvent {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data,
                }),
            ClientDboEvent::Updated(event_dbo) => ClientEvents::Updated(UpdatedEvent { by: event_dbo.by, at: event_dbo.at, data: event_dbo.data }),
            ClientDboEvent::Disable(event_dbo) => ClientEvents::Disabled(DisabledEvent { by: event_dbo.by, at: event_dbo.at, reason: event_dbo.reason })
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
            ClientEvents::Updated(updated) => ClientDboEvent::Updated(ClientUpdatedDbo { by: updated.by, at: updated.at, data: updated.data }),
            ClientEvents::Disabled(disabled) => ClientDboEvent::Disable(ClientDisabledDbo { by: disabled.by, at: disabled.at, reason: disabled.reason }),
        }
    }
}

