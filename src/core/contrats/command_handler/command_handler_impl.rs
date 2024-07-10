use async_trait::async_trait;

use crate::core::contrats::data::{ContratEvents, ContratStates, UpdatedEvent};
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::{CommandHandlerCreate, CommandHandlerUpdate};
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::{Error, ResultErr};

pub struct CreateContratHandler;

#[async_trait]
impl CommandHandlerCreate<ContratStates, ContratsCommands, ContratEvents> for CreateContratHandler {
    fn name(&self) -> String {
        "create-contrat".to_string()
    }

    async fn on_command(&self, _id: String, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Create(c) => Ok(
                ContratEvents::Created { by: context.subject.clone(), at: context.now, name: c.name }
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

pub struct UpdateContratHandler;
#[async_trait]
impl CommandHandlerUpdate<ContratStates, ContratsCommands, ContratEvents> for UpdateContratHandler {
    fn name(&self) -> String {
        "update-contrat".to_string()
    }

    async fn on_command(&self, _id: String, _state: ContratStates, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {

        match command {
            ContratsCommands::Update(c) => Ok(
                ContratEvents::Updated (UpdatedEvent {by: context.subject.clone(), at: context.now, name: c.name})
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
