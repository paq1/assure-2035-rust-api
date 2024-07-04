use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::shared::context::Context;
use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::id_generator::IdGenerator;
use crate::core::shared::repositories::{ReadOnlyEntityRepo, WriteOnlyEntityRepo, WriteOnlyEventRepo};
use crate::core::contrats::data::ContratStates::Contrat;
use crate::core::contrats::data::{ContratEvents, ContratStates, UpdatedEvent};
use crate::core::contrats::services::ContratService;
use crate::models::contrats::commands::{CreateContratCommand, DeleteContratCommand, UpdateContratCommand};
use crate::models::shared::errors::{Error, ResultErr};

pub struct ContratsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ContratStates, String> + ReadOnlyEntityRepo<ContratStates, String>,
    JOURNAL: WriteOnlyEventRepo<ContratEvents, String>,
{
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>,
}

#[async_trait]
impl<STORE, JOURNAL> ContratService for ContratsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ContratStates, String> + ReadOnlyEntityRepo<ContratStates, String> + Send,
    JOURNAL: WriteOnlyEventRepo<ContratEvents, String> + Send,
{
    async fn create_contrat(&self, command: CreateContratCommand, context: Context) -> ResultErr<String> {
        let entity_id = Self::generate_id();
        let event_id = Self::generate_id();

        let entity: Entity<ContratStates, String> = Entity {
            entity_id: entity_id.clone(),
            data: ContratStates::Contrat { name: command.name.clone() },
            version: None,
        };

        let event: EntityEvent<ContratEvents, String> = EntityEvent {
            entity_id: entity_id.clone(),
            event_id: event_id.clone(),
            data: ContratEvents::Created { by: context.subject, at: context.now, name: command.name.clone() },
        };


        Arc::clone(&self.journal)
            .lock().await
            .insert(event).await?;

        Arc::clone(&self.store)
            .lock().await
            .insert(entity).await
    }

    async fn update_contrat(&self, command: UpdateContratCommand, id: String, ctx: Context) -> ResultErr<String> {
        let current = self.store.lock().await.fetch_one(id.clone()).await?;

        match current {
            Some(entity) => {
                let event_id = Self::generate_id();

                let event: EntityEvent<ContratEvents, String> = EntityEvent {
                    entity_id: id.clone(),
                    event_id: event_id.clone(),
                    data: ContratEvents::Updated(UpdatedEvent { by: ctx.subject, at: ctx.now, name: command.name.clone() }),
                };

                let update_state = self.store.lock().await
                    .update(
                        id.clone(),
                        Entity {
                            data:
                            match entity.data.clone() {
                                _ => {
                                    Contrat {
                                        name: command.name
                                    }
                                }
                            },
                            ..entity.clone()
                        }
                    ).await;

                self.journal.lock().await.insert(event).await.and_then(|_| update_state)
            },
            None => Err(Error::Simple("not_found".to_string()))
        }
    }

    async fn delete_contrat(&self, _command: DeleteContratCommand, _id: String, _ctx: Context) -> ResultErr<String> {
        todo!()
    }
}

impl<STORE, JOURNAL> IdGenerator for ContratsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ContratStates, String> + ReadOnlyEntityRepo<ContratStates, String>,
    JOURNAL: WriteOnlyEventRepo<ContratEvents, String>
{
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
