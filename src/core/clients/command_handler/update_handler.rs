use async_trait::async_trait;

use crate::core::clients::data::events::{ClientEvents, UpdatedEvent};
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::CommandHandlerUpdate;
use crate::models::clients::commands::ClientsCommands;
use crate::models::shared::errors::{Error, ResultErr};

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
