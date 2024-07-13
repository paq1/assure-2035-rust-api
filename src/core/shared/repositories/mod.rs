use async_trait::async_trait;

use crate::core::shared::repositories::can_fetch_all::CanFetchAll;
use crate::core::shared::repositories::query::{InfoPaged, Page, Paged, Query};
use crate::models::shared::errors::ResultErr;

pub mod query;
pub mod can_fetch_all;
pub mod filter;
pub mod entities;
pub mod events;

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
                        number: query.pagination.page_number,
                        size: query.pagination.page_size,
                    },
                }
            }
        )
    }
}
