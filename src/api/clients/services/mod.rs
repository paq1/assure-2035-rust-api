use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::shared::context::Context;
use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::id_generator::IdGenerator;
use crate::core::shared::repositories::{ReadOnlyEntityRepo, WriteOnlyEntityRepo, WriteOnlyEventRepo};
use crate::core::clients::data::{ClientEvents, ClientStates, UpdatedEvent};
use crate::core::clients::data::ClientStates::Client;
use crate::core::clients::services::ClientService;
use crate::models::shared::errors::{Error, ResultErr};
use crate::models::clients::commands::*;

pub struct ClientsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ClientStates, String> + ReadOnlyEntityRepo<ClientStates, String>,
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String>,
{
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>,
}

#[async_trait]
impl<STORE, JOURNAL> ClientService for ClientsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ClientStates, String> + ReadOnlyEntityRepo<ClientStates, String> + Send,
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String> + Send,
{
    async fn create_client(&self, command: CreateClientCommand, context: Context) -> ResultErr<String> {

        // fixme mettre des erreurs standard: String -> CustomError / Failure
        let entity_id = Self::generate_id();
        let event_id = Self::generate_id();

        let entity: Entity<ClientStates, String> = Entity {
            entity_id: entity_id.clone(),
            data: ClientStates::Client { name: command.name.clone() },
            version: None,
        };

        let event: EntityEvent<ClientEvents, String> = EntityEvent {
            entity_id: entity_id.clone(),
            event_id: event_id.clone(),
            data: ClientEvents::Created { by: context.subject, at: context.now, name: command.name.clone() },
        };


        Arc::clone(&self.journal)
            .lock().await
            .insert(event).await?;

        Arc::clone(&self.store)
            .lock().await
            .insert(entity).await
    }

    async fn update_client(&self, command: UpdateClientCommand, id: String, ctx: Context) -> ResultErr<String> {
        let current = self.store.lock().await.fetch_one(id.clone()).await?;

        match current {
            Some(entity) => {
                let event_id = Self::generate_id();

                let event: EntityEvent<ClientEvents, String> = EntityEvent {
                    entity_id: id.clone(),
                    event_id: event_id.clone(),
                    data: ClientEvents::Updated(UpdatedEvent { by: ctx.subject, at: ctx.now, name: command.name.clone() }),
                };

                let update_state = self.store.lock().await
                    .update(
                        id.clone(),
                        Entity {
                            data:
                            match entity.data.clone() {
                                _ => {
                                    Client {
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

    async fn delete_client(&self, _command: DeleteClientCommand, _id: String, _ctx: Context) -> ResultErr<String> {
        todo!()
    }
}

impl<STORE, JOURNAL> IdGenerator for ClientsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ClientStates, String> + ReadOnlyEntityRepo<ClientStates, String>,
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String>
{
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
