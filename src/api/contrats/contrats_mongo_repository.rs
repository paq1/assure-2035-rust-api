use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::contrats::contrats_dbo::ContratDboState;
use crate::api::shared::daos::dbos::EntityDBO;
use crate::core::contrats::data::ContratStates;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::DAO;
use crate::core::shared::data::Entity;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEntityRepo, RepositoryEntity, WriteOnlyEntityRepo};
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::query::Query;
use crate::models::shared::errors::ResultErr;

pub struct ContratsMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EntityDBO<ContratDboState, String>, String>>>,
}

#[async_trait]
impl RepositoryEntity<ContratStates, String> for ContratsMongoRepository {}

#[async_trait]
impl CanFetchAll<Entity<ContratStates, String>> for ContratsMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<Entity<ContratStates, String>>> {
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
impl CanFetchMany<Entity<ContratStates, String>> for ContratsMongoRepository {}

#[async_trait]
impl ReadOnlyEntityRepo<ContratStates, String> for ContratsMongoRepository {
    async fn fetch_one(&self, id: String) -> ResultErr<Option<Entity<ContratStates, String>>> {
        self.dao
            .lock().await
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|dbo| dbo.into()))
    }
}

impl CanGetId<String> for EntityDBO<ContratDboState, String> {
    fn id(&self) -> &String {
        &self.entity_id
    }
}

#[async_trait]
impl WriteOnlyEntityRepo<ContratStates, String> for ContratsMongoRepository {
    async fn insert(&self, contrat: Entity<ContratStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<ContratDboState, String> = contrat.into();

        let sanitize_version: EntityDBO<ContratDboState, String> = EntityDBO {
            version: Some(0),
            ..entity_dbo.clone()
        };

        self.dao
            .lock().await
            .insert(sanitize_version).await
    }

    async fn update(&self, id: String, contrat: Entity<ContratStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<ContratDboState, String> = contrat.into();
        let sanitize_version: EntityDBO<ContratDboState, String> = EntityDBO {
            version: entity_dbo.clone().version.map(|old| old + 1),
            ..entity_dbo.clone()
        };

        self.dao
            .lock().await
            .update(id, sanitize_version).await
    }

    async fn delete(&self, id: String) -> ResultErr<String> {
        self.dao
            .lock().await
            .delete(id).await
    }
}
