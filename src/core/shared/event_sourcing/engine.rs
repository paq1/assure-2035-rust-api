use std::sync::Arc;
use futures::lock::Mutex;
use uuid::Uuid;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::context::Context;
use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::reducer::Reducer;
use crate::core::shared::repositories::{ReadOnlyEntityRepo, WriteOnlyEntityRepo, WriteOnlyEventRepo};
use crate::models::shared::errors::{Error, ResultErr};

pub struct Engine<
    STATE: Clone,
    COMMAND,
    EVENT,
    STORE,
    JOURNAL
>
where
    STORE: WriteOnlyEntityRepo<STATE, String> + ReadOnlyEntityRepo<STATE, String>,
    JOURNAL: WriteOnlyEventRepo<EVENT, String>,
{
    pub handlers: Vec<CommandHandler<STATE, COMMAND, EVENT>>,
    pub reducer: Reducer<EVENT, STATE>,
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>
}

impl<STATE, COMMAND, EVENT, STORE, JOURNAL> Engine<STATE, COMMAND, EVENT, STORE, JOURNAL>
where
    STATE: Clone,
    EVENT: Clone,
    STORE: WriteOnlyEntityRepo<STATE, String> + ReadOnlyEntityRepo<STATE, String>,
    JOURNAL: WriteOnlyEventRepo<EVENT, String>,
{
    pub async fn compute(&self, command: COMMAND, entity_id: String, name: String, context: Context) -> ResultErr<EVENT> {

        let command_handler_found = self.handlers
            .iter().find(|handler| {
            match handler {
                CommandHandler::Create(created) => created.name() == name,
                CommandHandler::Update(updated) => updated.name() == name
            }
        })
            .ok_or(Error::Simple("pas de gestionnaire pour cette commande".to_string()))?; // fixme changer l'erreur

        let maybe_entity = self.store.lock().await.fetch_one(entity_id.clone()).await?;
        let maybe_state = maybe_entity.clone().map(|entity| entity.data);

        let event = match command_handler_found {
            CommandHandler::Create(x) => x.on_command(entity_id.clone(), command, context).await,
            CommandHandler::Update(x) => {
                let state = maybe_state.clone().ok_or(Error::Simple("resource not found".to_string()))?;

                x.on_command(entity_id.clone(), state, command, context).await
            }
        }?;

        let new_state = (self.reducer.compute_new_state)(maybe_state, event.clone())
            .ok_or(Error::Simple("transition etat impossible".to_string()))?;
        let version = maybe_entity.clone()
            .map(|x| x.version.unwrap_or(0));
        let new_entity = Entity {
            entity_id: entity_id.clone(),
            data: new_state,
            version
        };

        if maybe_entity.is_none() {
            self.store.lock().await.insert(new_entity).await?;
        } else {
            self.store.lock().await.update(entity_id.clone(), new_entity).await?;
        }

        let event_entity = EntityEvent {
            entity_id: entity_id.clone(),
            event_id:  Self::generate_id(),
            data: event.clone()
        };
        self.journal.lock().await.insert(event_entity).await?;
        Ok(event)
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
