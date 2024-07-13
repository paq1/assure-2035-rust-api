use std::cmp::max;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::shared::context::Context;
use crate::core::shared::repositories::query::Paged;
use crate::models::shared::views::LinkView;

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
    pub fn new(paged: Paged<T>, ctx: &Context, ontology: String, other_link: HashMap<String, String>) -> Self {

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


        let query_first_without_prefix = ctx.filters
            .iter()
            .filter(|(k, _)| **k != "page[number]".to_string() && **k != "page[size]".to_string())
            .map(|(k, v)| {
                format!("{k}={v}")
            })
            .chain(vec!["page[number]=0".to_string(), format!("page[size]={}", paged.meta.page.size)])
            .collect::<Vec<String>>()
            .join("&");

        let query_first = if query_first_without_prefix.is_empty() {
            query_first_without_prefix
        } else {
            format!("?{query_first_without_prefix}")
        };

        let query_prev_without_prefix = ctx.filters
            .iter()
            .filter(|(k, _)| **k != "page[number]".to_string() && **k != "page[size]".to_string())
            .map(|(k, v)| {
                format!("{k}={v}")
            })
            .chain(vec![format!("page[number]={}", max(if paged.meta.page.number > 0 { paged.meta.page.number - 1 } else { 0 }, 0)), format!("page[size]={}", paged.meta.page.size)]) // fixme passer sur des isize pour la paginaition
            .collect::<Vec<String>>()
            .join("&");

        let query_prev = if query_prev_without_prefix.is_empty() {
            query_prev_without_prefix
        } else {
            format!("?{query_prev_without_prefix}")
        };


        let other_link_sanitize = other_link
            .into_iter()
            .map(|(k, v)| (k, format!("{external_url}/{v}")))
            .collect::<HashMap<String, String>>();



        let links = LinkView {
            links: HashMap::from([
                ("self".to_string(), format!("{external_url}/{ontology}")),
                ("last".to_string(), format!("{external_url}/{ontology}{}", query.clone())),
                ("prev".to_string(), format!("{external_url}/{ontology}{}", query_prev.clone())),
                ("first".to_string(), format!("{external_url}/{ontology}{query_first}")),
                ("next".to_string(), format!("{external_url}/{ontology}{}", query.clone())),
            ])
                .into_iter().chain(other_link_sanitize)
                .collect()
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