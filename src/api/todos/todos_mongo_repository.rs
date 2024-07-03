use async_trait::async_trait;

use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::todos::todo_dbo::TodoDboState;
use crate::api::todos::todos_mongo_dao::TodosMongoDAO;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::Entity;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEntityRepo, WriteOnlyEntityRepo};
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::query::Query;
use crate::core::todos::data::TodoStates;
use crate::models::shared::errors::ResultErr;

pub struct TodosMongoRepository {
    pub dao: TodosMongoDAO,
}

#[async_trait]
impl CanFetchAll<Entity<TodoStates, String>> for TodosMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<Entity<TodoStates, String>>> {
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
impl CanFetchMany<Entity<TodoStates, String>> for TodosMongoRepository {}

#[async_trait]
impl ReadOnlyEntityRepo<TodoStates, String> for TodosMongoRepository {
    async fn fetch_one(&self, id: String) -> ResultErr<Option<Entity<TodoStates, String>>> {
        self.dao
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|dbo| dbo.into()))
    }
}

impl CanGetId<String> for EntityDBO<TodoDboState, String> {
    fn id(&self) -> &String {
        &self.entity_id
    }
}

#[async_trait]
impl WriteOnlyEntityRepo<TodoStates, String> for TodosMongoRepository {
    async fn insert(&self, todo: Entity<TodoStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<TodoDboState, String> = todo.into();

        let sanitize_version: EntityDBO<TodoDboState, String> = EntityDBO {
            version: Some(0),
            ..entity_dbo.clone()
        };

        self.dao.insert(sanitize_version).await
    }

    async fn update(&self, id: String, todo: Entity<TodoStates, String>) -> ResultErr<String> {
        let entity_dbo: EntityDBO<TodoDboState, String> = todo.into();
        let sanitize_version: EntityDBO<TodoDboState, String> = EntityDBO {
            version: entity_dbo.clone().version.map(|old| old + 1),
            ..entity_dbo.clone()
        };

        self.dao.update(id, sanitize_version).await
    }

    async fn delete(&self, id: String) -> ResultErr<String> {
        self.dao.delete(id).await
    }
}
