use async_trait::async_trait;

use crate::api::contrats::contrats_dbo::ContratDboEvent;
use crate::api::contrats::contrats_mongo_dao::ContratsEventMongoDAO;
use crate::api::shared::daos::dbos::EventDBO;
use crate::core::contrats::data::ContratEvents;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::EntityEvent;
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::CanFetchMany;
use crate::core::shared::repositories::events::{ReadOnlyEventRepo, RepositoryEvents, WriteOnlyEventRepo};
use crate::core::shared::repositories::query::Query;
use crate::models::shared::errors::ResultErr;

pub struct ContratsEventMongoRepository {
    pub dao: ContratsEventMongoDAO,
}

#[async_trait]
impl RepositoryEvents<ContratEvents, String> for ContratsEventMongoRepository {}

#[async_trait]
impl CanFetchAll<EntityEvent<ContratEvents, String>> for ContratsEventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<ContratEvents, String>>> {
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
impl CanFetchMany<EntityEvent<ContratEvents, String>> for ContratsEventMongoRepository {}

#[async_trait]
impl ReadOnlyEventRepo<ContratEvents, String> for ContratsEventMongoRepository {
    async fn fetch_one(&self, event_id: String) -> ResultErr<Option<EntityEvent<ContratEvents, String>>> {
        self.dao.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                event_dbo.into()
            })
        })
    }
}

impl CanGetId<String> for EventDBO<ContratDboEvent, String> {
    fn id(&self) -> &String {
        &self.event_id
    }
}

#[async_trait]
impl WriteOnlyEventRepo<ContratEvents, String> for ContratsEventMongoRepository {
    async fn insert(&self, contrat: EntityEvent<ContratEvents, String>) -> ResultErr<String> {
        let dao: EventDBO<ContratDboEvent, String> = contrat.into();

        let dao_sanitize_version: EventDBO<ContratDboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.insert(dao_sanitize_version).await
    }
}