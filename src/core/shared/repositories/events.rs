use async_trait::async_trait;

use crate::core::shared::data::EntityEvent;
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::CanFetchMany;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait RepositoryEvents<DATA: Clone, ID: Clone>: ReadOnlyEventRepo<DATA, ID> + WriteOnlyEventRepo<DATA, ID> {}

#[async_trait]
pub trait ReadOnlyEventRepo<DATA: Clone, ID: Clone>: CanFetchAll<EntityEvent<DATA, ID>> + CanFetchMany<EntityEvent<DATA, ID>> + Sync + Send {
    async fn fetch_one(&self, id: &ID) -> ResultErr<Option<EntityEvent<DATA, ID>>>;
}

#[async_trait]
pub trait WriteOnlyEventRepo<DATA, ID> {
    async fn insert(&self, entity: &EntityEvent<DATA, ID>) -> ResultErr<ID>;
}