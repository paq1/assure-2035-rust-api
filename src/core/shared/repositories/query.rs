use std::collections::HashMap;

use crate::core::shared::repositories::filter::Filter;
use crate::models::shared::views::LinkView;

#[derive(Clone)]
pub struct Paged<T> {
    pub data: Vec<T>,
    pub meta: InfoPaged,
}

#[derive(Clone)]
pub struct Link {
    pub links: HashMap<String, String>,
}

impl From<Link> for LinkView {
    fn from(value: Link) -> Self {
        Self {
            links: value.links,
        }
    }
}

impl<T: Clone> Paged<T> {
    pub fn map<R, F>(
        &self,
        f: F,
    ) -> Paged<R>
    where
        F: Fn(T) -> R,
    {
        Paged {
            data: self
                .data.clone()
                .into_iter()
                .map(|data| f(data))
                .collect::<Vec<R>>(),
            meta: self.meta.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InfoPaged {
    pub total_pages: usize,
    pub total_records: usize,
    pub page: Page,
}

#[derive(Clone, Debug)]
pub struct Page {
    pub number: usize,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct Query {
    pub pagination: PaginationDef,
    pub filter: Filter,
}

#[derive(Clone, Debug)]
pub struct PaginationDef {
    pub page_number: usize,
    pub page_size: usize,
}

impl Default for PaginationDef {
    fn default() -> Self {
        Self {
            page_size: 10,
            page_number: 0,
        }
    }
}