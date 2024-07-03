use async_trait::async_trait;

use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::{CommandHandlerCreate, CommandHandlerUpdate};
use crate::core::todos::data::{TodoEvents, TodoStates, UpdatedEvent};
use crate::models::shared::errors::{Error, ResultErr};
use crate::models::todos::commands::TodoCommands;

pub struct CreateTodoHandler;

#[async_trait]
impl CommandHandlerCreate<TodoStates, TodoCommands, TodoEvents> for CreateTodoHandler {
    fn name(&self) -> String {
        "create".to_string()
    }

    async fn on_command(&self, _id: String, command: TodoCommands, context: Context) -> ResultErr<TodoEvents> {
        match command {
            TodoCommands::Create(c) => Ok(
                TodoEvents::Created { by: context.subject, at: context.now, name: c.name }
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

pub struct UpdateTodoHandler;
#[async_trait]
impl CommandHandlerUpdate<TodoStates, TodoCommands, TodoEvents> for UpdateTodoHandler {
    fn name(&self) -> String {
        "update".to_string()
    }

    async fn on_command(&self, _id: String, _state: TodoStates, command: TodoCommands, context: Context) -> ResultErr<TodoEvents> {

        match command {
            TodoCommands::Update(c) => Ok(
                TodoEvents::Updated (UpdatedEvent {by: context.subject, at: context.now, name: c.name})
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
