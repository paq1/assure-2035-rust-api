use crate::core::shared::repositories::filter::Filter;

pub struct Paged<T> {
    pub data: Vec<T>,
    pub meta: InfoPaged
}

pub struct InfoPaged {
    pub total_pages: usize,
    pub number: usize,
    pub size: usize,
}

#[derive(Clone)]
pub struct Query {
    pub pagination: PaginationDef,
    pub filter: Filter
}

#[derive(Clone)]
pub struct PaginationDef {
    pub page_number: usize,
    pub page_size: usize
}