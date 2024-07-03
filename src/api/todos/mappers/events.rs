use crate::api::shared::daos::dbos::EventDBO;
use crate::api::todos::todo_dbo::{TodoDboEvent, TodoUpdatedDbo};
use crate::core::shared::data::EntityEvent;
use crate::core::todos::data::{TodoEvents, UpdatedEvent};

impl From<TodoDboEvent> for TodoEvents {
    fn from(value: TodoDboEvent) -> Self {
        match value {
            TodoDboEvent::TodoCreatedDbo { by, at, name } => TodoEvents::Created { by, at, name },
            TodoDboEvent::Updated(event_dbo) => TodoEvents::Updated(UpdatedEvent { by: event_dbo.by, at: event_dbo.at, name: event_dbo.name })
        }
    }
}

impl From<EventDBO<TodoDboEvent, String>> for EntityEvent<TodoEvents, String> {
    fn from(value: EventDBO<TodoDboEvent, String>) -> Self {
        EntityEvent {
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone(),
        }
    }
}


impl From<EntityEvent<TodoEvents, String>> for EventDBO<TodoDboEvent, String> {
    fn from(value: EntityEvent<TodoEvents, String>) -> Self {
        EventDBO {
            id_mongo: None,
            version: None,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone(),
        }
    }
}

impl From<TodoEvents> for TodoDboEvent {
    fn from(value: TodoEvents) -> Self {
        match value {
            TodoEvents::Created { by, at, name } => TodoDboEvent::TodoCreatedDbo { by, at, name },
            TodoEvents::Updated(updated) => TodoDboEvent::Updated(TodoUpdatedDbo { by: updated.by, at: updated.at, name: updated.name })
        }
    }
}

