use async_trait::async_trait;

use crate::core::contrats::data::{ContratEvents, ContratStates, RejectEvent, UserInfo};
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::CommandHandlerUpdate;
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::{Error, ResultErr};

pub struct RejectContractHandler {}
#[async_trait]
impl CommandHandlerUpdate<ContratStates, ContratsCommands, ContratEvents> for RejectContractHandler {
    fn name(&self) -> String {
        Self::get_name().to_string()
    }

    async fn on_command(&self, _id: String, _state: ContratStates, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Reject(cmd) => Ok(
                ContratEvents::Rejected(
                    RejectEvent {
                        reject_by: UserInfo {
                            uid: context.subject.clone(),
                            display_name: context.name.clone(),
                            email: context.email.clone(),
                        },
                        by: context.subject.clone(),
                        at: context.now,
                        comment: cmd.comment
                    }
                )
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl<'a> RejectContractHandler {
    pub fn get_name() -> &'a str {
        "reject-contract"
    }
}