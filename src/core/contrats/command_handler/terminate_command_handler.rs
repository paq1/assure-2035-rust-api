use async_trait::async_trait;

use crate::core::contrats::data::{ContratEvents, ContratStates, TerminatedEvent};
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::CommandHandlerUpdate;
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::{Error, ResultErr};

pub struct TerminateContractHandler {}
#[async_trait]
impl CommandHandlerUpdate<ContratStates, ContratsCommands, ContratEvents> for TerminateContractHandler {
    fn name(&self) -> String {
        Self::get_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: ContratStates, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Terminate(cmd) => Ok(
                ContratEvents::Terminated(
                    TerminatedEvent {
                        by: context.subject.clone(),
                        at: context.now,
                        reason: cmd.reason,
                    }
                )
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl<'a> TerminateContractHandler {
    pub fn get_name() -> &'a str {
        "terminate-contract"
    }
}