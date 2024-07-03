use async_trait::async_trait;

use crate::api::shared::daos::dbos::EventDBO;
use crate::api::clients::clients_dbo::ClientDboEvent;
use crate::api::clients::clients_mongo_dao::ClientsEventMongoDAO;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::EntityEvent;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEventRepo, WriteOnlyEventRepo};
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::query::Query;
use crate::core::clients::data::ClientEvents;
use crate::models::shared::errors::ResultErr;

pub struct ClientsEventMongoRepository {
    pub dao: ClientsEventMongoDAO,
}

#[async_trait]
impl CanFetchAll<EntityEvent<ClientEvents, String>> for ClientsEventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<ClientEvents, String>>> {
        self.dao
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
        self.dao.fetch_one(event_id).await.map(|maybevent| {
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

        self.dao.insert(dao_sanitize_version).await
    }
}