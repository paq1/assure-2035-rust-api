use async_trait::async_trait;

use crate::core::shared::data::Entity;
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::CanFetchMany;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait RepositoryEntity<DATA: Clone, ID: Clone>: ReadOnlyEntityRepo<DATA, ID> + WriteOnlyEntityRepo<DATA, ID> {}

#[async_trait]
pub trait ReadOnlyEntityRepo<DATA: Clone, ID: Clone>: CanFetchAll<Entity<DATA, ID>> + CanFetchMany<Entity<DATA, ID>> + Sync + Send {
    async fn fetch_one(&self, id: ID) -> ResultErr<Option<Entity<DATA, ID>>>;
}

#[async_trait]
pub trait WriteOnlyEntityRepo<DATA: Clone, ID: Clone> {
    async fn insert(&self, entity: Entity<DATA, ID>) -> ResultErr<ID>;
    async fn update(&self, id: ID, entity: Entity<DATA, ID>) -> ResultErr<ID>;
    async fn delete(&self, id: ID) -> ResultErr<ID>;
}
