use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::shared::context::Context;
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
    pub fn new(paged: Paged<T>, ctx: &Context, ontology: String) -> Self {

        let external_url = ctx.meta
            .get("externalUrl")
            .map(|urlref| urlref.clone())
            .unwrap_or("unknown".to_string());


        let query_without_prefix = ctx.filters
            .iter()
            .map(|(k, v)| {
                format!("{k}={v}")
            })
            .collect::<Vec<String>>()
            .join("&");

        let query = if query_without_prefix.is_empty() {
            query_without_prefix
        } else {
            format!("?{query_without_prefix}")
        };

        let links = LinkView {
            links: HashMap::from([
                ("self".to_string(), format!("{external_url}/{ontology}{}", query.clone())),
                ("last".to_string(), format!("{external_url}/{ontology}{}", query.clone())),
                ("prev".to_string(), format!("{external_url}/{ontology}{}", query.clone())),
                ("first".to_string(), format!("{external_url}/{ontology}{}", query.clone())),
                ("next".to_string(), format!("{external_url}/{ontology}{}", query.clone())),
            ]) // fixme mettre les bonnes valeures ici
        };

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
            links: Some(links), // fixme passer les info link ici (c'est uniquement lié a la view)
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