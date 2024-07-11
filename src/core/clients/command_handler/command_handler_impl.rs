use async_trait::async_trait;

use crate::core::clients::data::{ClientEvents, CreatedEvent, DisabledEvent, UpdatedEvent};
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::{CommandHandlerCreate, CommandHandlerUpdate};
use crate::models::clients::commands::ClientsCommands;
use crate::models::shared::errors::{Error, ResultErr};

pub struct CreateClientHandler;

#[async_trait]
impl CommandHandlerCreate<ClientStates, ClientsCommands, ClientEvents> for CreateClientHandler {
    fn name(&self) -> String {
        "create-client".to_string()
    }

    async fn on_command(&self, _id: String, command: ClientsCommands, context: &Context) -> ResultErr<ClientEvents> {
        match command {
            ClientsCommands::Create(c) => Ok(
                ClientEvents::Created(CreatedEvent {
                    by: context.subject.clone(),
                    at: context.now,
                    data: c.data
                })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

pub struct UpdateClientHandler;
#[async_trait]
impl CommandHandlerUpdate<ClientStates, ClientsCommands, ClientEvents> for UpdateClientHandler {
    fn name(&self) -> String {
        "update-client".to_string()
    }

    async fn on_command(&self, _id: String, _state: ClientStates, command: ClientsCommands, context: &Context) -> ResultErr<ClientEvents> {
        match command {
            ClientsCommands::Update(c) => Ok(
                ClientEvents::Updated(UpdatedEvent { by: context.subject.clone(), at: context.now, data: c.data })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

pub struct DisableClientHandler;
#[async_trait]
impl CommandHandlerUpdate<ClientStates, ClientsCommands, ClientEvents> for DisableClientHandler {
    fn name(&self) -> String {
        "disable-client".to_string()
    }

    async fn on_command(&self, _id: String, _state: ClientStates, command: ClientsCommands, context: &Context) -> ResultErr<ClientEvents> {
        match command {
            ClientsCommands::Disable(cmd) => Ok(
                ClientEvents::Disabled(DisabledEvent { by: context.subject.clone(), at: context.now, reason: cmd.reason })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
