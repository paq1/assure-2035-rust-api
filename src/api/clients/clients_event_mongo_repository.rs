use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::clients::clients_dbo::ClientDboEvent;
use crate::api::shared::daos::dbos::EventDBO;
use crate::core::clients::data::events::ClientEvents;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::DAO;
use crate::core::shared::data::EntityEvent;
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::CanFetchMany;
use crate::core::shared::repositories::events::{ReadOnlyEventRepo, RepositoryEvents, WriteOnlyEventRepo};
use crate::core::shared::repositories::query::Query;
use crate::models::shared::errors::ResultErr;

pub struct ClientsEventMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EventDBO<ClientDboEvent, String>, String>>>,
}

#[async_trait]
impl RepositoryEvents<ClientEvents, String> for ClientsEventMongoRepository {}

#[async_trait]
impl CanFetchAll<EntityEvent<ClientEvents, String>> for ClientsEventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<ClientEvents, String>>> {
        self.dao
            .lock().await
            .fetch_all(query)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| dbo.into())
                    .collect()
            })
    }
}

#[async_trait]
impl CanFetchMany<EntityEvent<ClientEvents, String>> for ClientsEventMongoRepository {}

#[async_trait]
impl ReadOnlyEventRepo<ClientEvents, String> for ClientsEventMongoRepository {
    async fn fetch_one(&self, event_id: String) -> ResultErr<Option<EntityEvent<ClientEvents, String>>> {
        self.dao.lock().await.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                event_dbo.into()
            })
        })
    }
}

impl CanGetId<String> for EventDBO<ClientDboEvent, String> {
    fn id(&self) -> &String {
        &self.event_id
    }
}

#[async_trait]
impl WriteOnlyEventRepo<ClientEvents, String> for ClientsEventMongoRepository {
    async fn insert(&self, client: EntityEvent<ClientEvents, String>) -> ResultErr<String> {
        let dao: EventDBO<ClientDboEvent, String> = client.into();

        let dao_sanitize_version: EventDBO<ClientDboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.lock().await.insert(dao_sanitize_version).await
    }
}