use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use crate::core::shared::repositories::filter::Filter;
use crate::core::shared::repositories::query::PaginationDef;

#[derive(Serialize, Deserialize, IntoParams, Debug)]
pub struct TodoQuery {
    #[serde(rename="page[number]")]
    pub number: Option<usize>,
    #[serde(rename="page[size]")]
    pub size: Option<usize>
}

impl From<Query<TodoQuery>> for crate::core::shared::repositories::query::Query {
    fn from(value: Query<TodoQuery>) -> Self {
        let size = value.size.unwrap_or(10);
        let number = value.number.unwrap_or(1);

        Self {
            pagination: PaginationDef {
                page_number: number,
                page_size: size
            },
            filter: Filter::None
        }
    }
}