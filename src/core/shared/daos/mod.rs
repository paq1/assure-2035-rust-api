use async_trait::async_trait;
use crate::core::shared::repositories::query::Query;
use crate::models::shared::errors::ResultErr;

pub trait DAO<DBO, ID>: ReadOnlyDAO<DBO, ID> + WriteOnlyDAO<DBO, ID> + Sync + Send {}

#[async_trait]
pub trait ReadOnlyDAO<DBO, ID> {
    async fn fetch_one(&self, id: ID) -> ResultErr<Option<DBO>>;
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<DBO>>;
}

#[async_trait]
pub trait WriteOnlyDAO<DBO, ID> {
    async fn insert(&self, entity: DBO) -> ResultErr<ID>;
    async fn update(&self, id: ID, entity: DBO) -> ResultErr<ID>;
    async fn delete(&self, id: ID) -> ResultErr<ID>;
}

