use async_trait::async_trait;

use crate::core::clients::data::events::{ClientEvents, CreatedEvent};
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::CommandHandlerCreate;
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
                    data: c.data,
                })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}