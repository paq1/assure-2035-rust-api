use async_trait::async_trait;

use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::clients::clients_dbo::ClientDboState;
use crate::api::clients::clients_mongo_dao::ClientsMongoDAO;
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::Entity;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEntityRepo, WriteOnlyEntityRepo};
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::query::Query;
use crate::models::shared::errors::ResultErr;

pub struct ClientsMongoRepository {
    pub dao: ClientsMongoDAO,
}

#[async_trait]
impl CanFetchAll<Entity<ClientStates, String>> for ClientsMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<Entity<ClientStates, String>>> {
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
impl CanFetchMany<Entity<ClientStates, String>> for ClientsMongoRepository {}

#[async_trait]
impl ReadOnlyEntityRepo<ClientStates, String> for ClientsMongoRepository {
    async fn fetch_one(&self, id: String) -> ResultErr<Option<Entity<ClientStates, String>>> {
        self.dao
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|dbo| dbo.into()))
    }
}

impl CanGetId<String> for EntityDBO<ClientDboState, String> {
    fn id(&self) -> &String {
        &self.entity_id
    }
}

#[async_trait]
impl WriteOnlyEntityRepo<ClientStates, String> for ClientsMongoRepository {
    async fn insert(&self, client: Entity<ClientStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<ClientDboState, String> = client.into();

        let sanitize_version: EntityDBO<ClientDboState, String> = EntityDBO {
            version: Some(0),
            ..entity_dbo.clone()
        };

        self.dao.insert(sanitize_version).await
    }

    async fn update(&self, id: String, client: Entity<ClientStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<ClientDboState, String> = client.into();
        let sanitize_version: EntityDBO<ClientDboState, String> = EntityDBO {
            version: entity_dbo.clone().version.map(|old| old + 1),
            ..entity_dbo.clone()
        };

        self.dao.update(id, sanitize_version).await
    }

    async fn delete(&self, id: String) -> ResultErr<String> {
        self.dao.delete(id).await
    }
}
