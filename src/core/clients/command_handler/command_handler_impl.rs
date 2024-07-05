use async_trait::async_trait;

use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::{CommandHandlerCreate, CommandHandlerUpdate};
use crate::core::clients::data::{ClientEvents, ClientStates, CreatedEvent, UpdatedEvent};
use crate::models::shared::errors::{Error, ResultErr};
use crate::models::clients::commands::ClientsCommands;

pub struct CreateClientHandler;

#[async_trait]
impl CommandHandlerCreate<ClientStates, ClientsCommands, ClientEvents> for CreateClientHandler {
    fn name(&self) -> String {
        "create-client".to_string()
    }

    async fn on_command(&self, _id: String, command: ClientsCommands, context: Context) -> ResultErr<ClientEvents> {
        match command {
            ClientsCommands::Create(c) => Ok(
                ClientEvents::Created(CreatedEvent {
                    by: context.subject,
                    at: context.now,
                    first_name: c.first_name,
                    last_name: c.last_name,
                    birth_date: c.birth_date,
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

    async fn on_command(&self, _id: String, _state: ClientStates, command: ClientsCommands, context: Context) -> ResultErr<ClientEvents> {
        match command {
            ClientsCommands::Update(c) => Ok(
                ClientEvents::Updated(UpdatedEvent { by: context.subject, at: context.now, name: c.name })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
