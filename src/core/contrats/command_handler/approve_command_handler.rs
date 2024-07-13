use async_trait::async_trait;

use crate::core::contrats::data::{ApprovedEvent, ContratEvents, ContratStates};
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::CommandHandlerUpdate;
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::{Error, ResultErr};

pub struct ApproveContractHandler;
#[async_trait]
impl CommandHandlerUpdate<ContratStates, ContratsCommands, ContratEvents> for ApproveContractHandler {
    fn name(&self) -> String {
        Self::get_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: ContratStates, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Approve(_) => Ok(
                ContratEvents::Approved(ApprovedEvent { by: context.subject.clone(), at: context.now })
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl<'a> ApproveContractHandler {
    pub fn get_name() -> &'a str {
        "approve-contract"
    }
}