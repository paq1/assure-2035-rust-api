use async_trait::async_trait;

use crate::core::contrats::data::{ContratEvents, ContratStates, RefusedEvent, UserInfo};
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::CommandHandlerUpdate;
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::{Error, ResultErr};

pub struct RefuseContractHandler;
#[async_trait]
impl CommandHandlerUpdate<ContratStates, ContratsCommands, ContratEvents> for RefuseContractHandler {
    fn name(&self) -> String {
        Self::get_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: ContratStates, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Refuse(_) => Ok(
                ContratEvents::Refused(
                    RefusedEvent {
                        refused_by: UserInfo {
                            uid: context.subject.clone(),
                            display_name: context.name.clone(),
                            email: context.email.clone(),
                        },
                        by: context.subject.clone(),
                        at: context.now
                    }
                )
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl<'a> RefuseContractHandler {
    pub fn get_name() -> &'a str {
        "refuse-contract"
    }
}