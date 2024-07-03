use async_trait::async_trait;

use crate::api::shared::daos::dbos::EventDBO;
use crate::api::todos::todo_dbo::TodoDboEvent;
use crate::api::todos::todos_mongo_dao::TodosEventMongoDAO;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::EntityEvent;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEventRepo, WriteOnlyEventRepo};
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::query::Query;
use crate::core::todos::data::TodoEvents;
use crate::models::shared::errors::ResultErr;

pub struct TodosEventMongoRepository {
    pub dao: TodosEventMongoDAO,
}

#[async_trait]
impl CanFetchAll<EntityEvent<TodoEvents, String>> for TodosEventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<TodoEvents, String>>> {
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
impl CanFetchMany<EntityEvent<TodoEvents, String>> for TodosEventMongoRepository {}

#[async_trait]
impl ReadOnlyEventRepo<TodoEvents, String> for TodosEventMongoRepository {
    async fn fetch_one(&self, event_id: String) -> ResultErr<Option<EntityEvent<TodoEvents, String>>> {
        self.dao.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                event_dbo.into()
            })
        })
    }
}

impl CanGetId<String> for EventDBO<TodoDboEvent, String> {
    fn id(&self) -> &String {
        &self.event_id
    }
}

#[async_trait]
impl WriteOnlyEventRepo<TodoEvents, String> for TodosEventMongoRepository {
    async fn insert(&self, todo: EntityEvent<TodoEvents, String>) -> ResultErr<String> {
        let dao: EventDBO<TodoDboEvent, String> = todo.into();

        let dao_sanitize_version: EventDBO<TodoDboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.insert(dao_sanitize_version).await
    }
}