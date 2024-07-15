use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::clients::clients_dbo::ClientDboState;
use crate::api::shared::daos::dbos::EntityDBO;
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::DAO;
use crate::core::shared::data::Entity;
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::CanFetchMany;
use crate::core::shared::repositories::entities::{ReadOnlyEntityRepo, RepositoryEntity, WriteOnlyEntityRepo};
use crate::core::shared::repositories::query::Query;
use crate::models::shared::errors::ResultErr;

pub struct ClientsMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EntityDBO<ClientDboState, String>, String>>>,
}

#[async_trait]
impl RepositoryEntity<ClientStates, String> for ClientsMongoRepository {}

#[async_trait]
impl CanFetchAll<Entity<ClientStates, String>> for ClientsMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<Entity<ClientStates, String>>> {
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
impl CanFetchMany<Entity<ClientStates, String>> for ClientsMongoRepository {}

#[async_trait]
impl ReadOnlyEntityRepo<ClientStates, String> for ClientsMongoRepository {
    async fn fetch_one(&self, id: &String) -> ResultErr<Option<Entity<ClientStates, String>>> {
        self.dao
            .lock().await
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
    async fn insert(&self, client: &Entity<ClientStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<ClientDboState, String> = client.clone().into();

        let sanitize_version: EntityDBO<ClientDboState, String> = EntityDBO {
            version: Some(0),
            ..entity_dbo
        };

        self.dao
            .lock().await
            .insert(&sanitize_version).await
    }

    async fn update(&self, id: &String, client: &Entity<ClientStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<ClientDboState, String> = client.clone().into();
        let sanitize_version: EntityDBO<ClientDboState, String> = EntityDBO {
            version: entity_dbo.version.map(|old| old + 1),
            ..entity_dbo
        };

        self.dao
            .lock().await
            .update(id, &sanitize_version).await
    }

    async fn delete(&self, id: &String) -> ResultErr<String> {
        self.dao
            .lock().await
            .delete(id).await
    }
}
