use async_trait::async_trait;

use crate::core::shared::context::Context;
use crate::models::shared::errors::ResultErr;

pub mod engine;

pub enum CommandHandler<STATE, COMMAND, EVT> {
    Create(Box<dyn CommandHandlerCreate<STATE, COMMAND, EVT>>),
    Update(Box<dyn CommandHandlerUpdate<STATE, COMMAND, EVT>>),
}

#[async_trait]
pub trait CommandHandlerCreate<STATE, COMMAND, EVT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(&self, id: String, command: COMMAND, context: &Context) -> ResultErr<EVT>;
}

#[async_trait]
pub trait CommandHandlerUpdate<STATE, COMMAND, EVT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(&self, id: String, state: STATE, command: COMMAND, context: &Context) -> ResultErr<EVT>;
}
