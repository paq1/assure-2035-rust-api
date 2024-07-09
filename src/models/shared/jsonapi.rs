use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::shared::repositories::query::Paged;
use crate::models::shared::views::command_handler_view::LinkView;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ManyView<T>
where
    T: Serialize + Clone,
{
    #[schema(example = "[]")]
    pub data: Vec<T>,
    pub meta: Option<PaginationView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkView>
}

impl<T: Serialize + Clone> ManyView<T> {
    pub fn new(paged: Paged<T>) -> Self {
        Self {
            data: paged.data,
            meta: Some(
                PaginationView {
                    total_pages: paged.meta.total_pages,
                    total_records: paged.meta.total_records,
                    page: PageView {
                        number: paged.meta.page.number,
                        size: paged.meta.page.size
                    }
                }
            ),
            links: paged.links.map(|link| {
                link.into()
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct PaginationView {
    #[serde(rename = "totalPages")]
    pub total_pages: usize,
    #[serde(rename = "totalRecords")]
    pub total_records: usize,
    pub page: PageView
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct PageView {
    pub number: usize,
    pub size: usize,
}

pub trait CanBeView<DATAVIEW> {
    fn to_view(&self) -> DATAVIEW;
}

pub trait CanGetTypee {
    fn get_type(&self) -> String;
}