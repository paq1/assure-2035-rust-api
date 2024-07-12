use async_trait::async_trait;

use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::query::{InfoPaged, Page, Paged, Query};
use crate::models::shared::errors::ResultErr;

pub mod query;
pub mod can_fetch_all;
pub mod filter;

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

#[async_trait]
pub trait ReadOnlyEventRepo<DATA: Clone, ID: Clone>: CanFetchAll<EntityEvent<DATA, ID>> + CanFetchMany<EntityEvent<DATA, ID>> {
    async fn fetch_one(&self, id: ID) -> ResultErr<Option<EntityEvent<DATA, ID>>>;
}

#[async_trait]
pub trait WriteOnlyEventRepo<DATA, ID> {
    async fn insert(&self, entity: EntityEvent<DATA, ID>) -> ResultErr<ID>;
}

#[async_trait]
pub trait CanFetchMany<ENTITY: Clone>: CanFetchAll<ENTITY> {
    async fn fetch_many(&self, query: Query) -> ResultErr<Paged<ENTITY>> {
        let entities = self.fetch_all(query.clone()).await?;
        let total_records = entities.len();
        let start = query.pagination.page_number * query.pagination.page_size;
        let end = start.clone() + query.pagination.page_size;
        let max_page = f64::ceil(entities.len() as f64 / query.pagination.page_size as f64) as usize;

        let paged_entities = if entities.is_empty() {
            vec![]
        } else {
            if start > entities.len() {
                vec![]
            } else {
                let sanitize_end = if end > total_records {
                    total_records
                } else {
                    end
                };
                entities.clone().iter().as_slice()[start..sanitize_end].to_vec()
            }
        };

        Ok(
            Paged {
                data: paged_entities,
                meta: InfoPaged {
                    total_pages: max_page,
                    total_records,
                    page: Page {
                        number: query.pagination.page_number - 1,
                        size: query.pagination.page_size,
                    },
                }
            }
        )
    }
}
