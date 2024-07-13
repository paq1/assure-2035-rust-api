use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::core::contrats::data::{ContratEvents, ContratStates, CreatedEvent, UpdatedEvent};
use crate::core::contrats::services::ContratService;
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::{CommandHandlerCreate, CommandHandlerUpdate};
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::{Error, ResultErr};

pub struct CreateContratHandler {
    pub contract_service: Arc<Mutex<dyn ContratService>>
}

#[async_trait]
impl CommandHandlerCreate<ContratStates, ContratsCommands, ContratEvents> for CreateContratHandler {
    fn name(&self) -> String {
        "create-contrat".to_string()
    }

    async fn on_command(&self, _id: String, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Create(c) => Ok(
                ContratEvents::Created (
                    CreatedEvent {
                        by: context.subject.clone(),
                        at: context.now,
                        data: c.data.clone(),
                        premium: self.contract_service.lock().await.calcul_premium(c).await?
                    }
                )
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
                ContratEvents::Updated (UpdatedEvent {by: context.subject.clone(), at: context.now, data: c.data})
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}
