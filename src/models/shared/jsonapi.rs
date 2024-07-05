use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::shared::repositories::query::Paged;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Many<T>
where
    T: Serialize + Clone,
{
    #[schema(example = "[]")]
    pub data: Vec<T>,
    pub meta: Option<Pagination>,
}

impl<T: Serialize + Clone> Many<T> {
    pub fn new(paged: Paged<T>) -> Self {
        Self {
            data: paged.data,
            meta: Some(
                Pagination {
                    total_pages: paged.meta.total_pages,
                    number: paged.meta.number,
                    size: paged.meta.size
                }
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Pagination {
    #[serde(rename = "totalPages")]
    pub total_pages: usize,
    pub number: usize,
    pub size: usize,
}


pub trait CanBeView<DATAVIEW> {
    fn to_view(&self) -> DATAVIEW;
}

