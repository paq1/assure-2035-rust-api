use crate::core::shared::repositories::filter::Filter;

#[derive(Clone)]
pub struct Paged<T> {
    pub data: Vec<T>,
    pub meta: InfoPaged,
}

impl<T: Clone> Paged<T> {
    pub fn map<R>(&self, x: fn(T) -> R) -> Paged<R> {
        Paged {
            data: self
                .data.clone()
                .into_iter()
                .map(|data| x(data))
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