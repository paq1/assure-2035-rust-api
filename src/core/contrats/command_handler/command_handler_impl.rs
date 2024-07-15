use std::sync::Arc;

use async_trait::async_trait;

use crate::core::contrats::data::{ContratEvents, ContratStates, CreatedEvent, UpdatedEvent};
use crate::core::contrats::services::ContratService;
use crate::core::shared::context::Context;
use crate::core::shared::event_sourcing::{CommandHandlerCreate, CommandHandlerUpdate};
use crate::models::contrats::commands::ContratsCommands;
use crate::models::contrats::shared::ContractData;
use crate::models::shared::errors::{Error, ResultErr};

pub struct CreateContratHandler {
    pub contract_service: Arc<dyn ContratService>,
}

#[async_trait]
impl CommandHandlerCreate<ContratStates, ContratsCommands, ContratEvents> for CreateContratHandler {
    fn name(&self) -> String {
        "create-contrat".to_string()
    }

    async fn on_command(&self, _id: String, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Create(c) => Ok(
                ContratEvents::Created(
                    CreatedEvent {
                        by: context.subject.clone(),
                        at: context.now,
                        data: c.data.clone(),
                        premium: self.contract_service.calcul_premium(&c.data).await?,
                    }
                )
            ),
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

pub struct UpdateContratHandler {
    pub contract_service: Arc<dyn ContratService>,
}
#[async_trait]
impl CommandHandlerUpdate<ContratStates, ContratsCommands, ContratEvents> for UpdateContratHandler {
    fn name(&self) -> String {
        "update-contrat".to_string()
    }

    async fn on_command(&self, _id: String, state: ContratStates, command: ContratsCommands, context: &Context) -> ResultErr<ContratEvents> {
        match command {
            ContratsCommands::Update(c) => {
                let new_data = ContractData {
                    holder: Self::get_holder(&state),
                    product: c.product.clone(),
                    formula: c.formula.clone(),
                    vehicle: c.vehicle.clone()
                };

                Ok(
                    ContratEvents::Updated(UpdatedEvent {
                        by: context.subject.clone(),
                        at: context.now,
                        product: c.product,
                        formula: c.formula,
                        vehicle: c.vehicle,
                        premium: self.contract_service.calcul_premium(&new_data).await?,
                    })
                )
            },
            _ => Err(Error::Simple("bad request".to_string()))
        }
    }
}

impl UpdateContratHandler {
    fn get_holder(state: &ContratStates) -> String {
        match state {
            ContratStates::Pending(p) => p.data.holder.clone(),
            ContratStates::PendingAmendment(p) => p.data.holder.clone(),
            ContratStates::Actif(a) => a.data.holder.clone(),
            ContratStates::Inactif(i) => i.data.holder.clone(),
        }
    }
}
