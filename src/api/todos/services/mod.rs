use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::shared::context::Context;
use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::id_generator::IdGenerator;
use crate::core::shared::repositories::{ReadOnlyEntityRepo, WriteOnlyEntityRepo, WriteOnlyEventRepo};
use crate::core::todos::data::{TodoEvents, TodoStates, UpdatedEvent};
use crate::core::todos::data::TodoStates::Todo;
use crate::core::todos::services::TodosService;
use crate::models::shared::errors::{Error, ResultErr};
use crate::models::todos::commands::*;

pub struct TodosServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<TodoStates, String> + ReadOnlyEntityRepo<TodoStates, String>,
    JOURNAL: WriteOnlyEventRepo<TodoEvents, String>,
{
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>,
}

#[async_trait]
impl<STORE, JOURNAL> TodosService for TodosServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<TodoStates, String> + ReadOnlyEntityRepo<TodoStates, String> + Send,
    JOURNAL: WriteOnlyEventRepo<TodoEvents, String> + Send,
{
    async fn create_todo(&self, command: CreateTodoCommand, context: Context) -> ResultErr<String> {

        // fixme mettre des erreurs standard: String -> CustomError / Failure
        let entity_id = Self::generate_id();
        let event_id = Self::generate_id();

        let entity: Entity<TodoStates, String> = Entity {
            entity_id: entity_id.clone(),
            data: TodoStates::Todo { name: command.name.clone() },
            version: None,
        };

        let event: EntityEvent<TodoEvents, String> = EntityEvent {
            entity_id: entity_id.clone(),
            event_id: event_id.clone(),
            data: TodoEvents::Created { by: context.subject, at: context.now, name: command.name.clone() },
        };


        Arc::clone(&self.journal)
            .lock().await
            .insert(event).await?;

        Arc::clone(&self.store)
            .lock().await
            .insert(entity).await
    }

    async fn update_todo(&self, command: UpdateTodoCommand, id: String, ctx: Context) -> ResultErr<String> {
        let current = self.store.lock().await.fetch_one(id.clone()).await?;

        match current {
            Some(entity) => {
                let event_id = Self::generate_id();

                let event: EntityEvent<TodoEvents, String> = EntityEvent {
                    entity_id: id.clone(),
                    event_id: event_id.clone(),
                    data: TodoEvents::Updated(UpdatedEvent { by: ctx.subject, at: ctx.now, name: command.name.clone() }),
                };

                let update_state = self.store.lock().await
                    .update(
                        id.clone(),
                        Entity {
                            data:
                            match entity.data.clone() {
                                _ => {
                                    Todo {
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

    async fn delete_todo(&self, _command: DeleteTodoCommand, _id: String, _ctx: Context) -> ResultErr<String> {
        todo!()
    }
}

impl<STORE, JOURNAL> IdGenerator for TodosServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<TodoStates, String> + ReadOnlyEntityRepo<TodoStates, String>,
    JOURNAL: WriteOnlyEventRepo<TodoEvents, String>
{
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
