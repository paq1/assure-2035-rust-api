use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::todos::todo_dbo::TodoDboState;
use crate::core::shared::data::Entity;
use crate::core::todos::data::TodoStates;

impl From<TodoDboState> for TodoStates {
    fn from(value: TodoDboState) -> Self {
        match value {
            TodoDboState::TodoDbo { name } => TodoStates::Todo {
                name: name.clone()
            }
        }
    }
}


impl From<Entity<TodoStates, String>> for EntityDBO<TodoDboState, String> {
    fn from(value: Entity<TodoStates, String>) -> Self {
        EntityDBO {
            id_mongo: None,
            version: value.version,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
        }
    }
}

impl From<TodoStates> for TodoDboState {
    fn from(value: TodoStates) -> Self {
        match value {
            TodoStates::Todo { name } => TodoDboState::TodoDbo { name: name.clone() }
        }
    }
}

impl From<EntityDBO<TodoDboState, String>> for Entity<TodoStates, String> {
    fn from(value: EntityDBO<TodoDboState, String>) -> Self {
        Self {
            entity_id: value.entity_id,
            data: value.data.into(),
            version: value.version,
        }
    }
}