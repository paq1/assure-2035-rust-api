use std::cmp::max;
use std::collections::HashMap;

use serde::Serialize;

use crate::core::shared::context::Context;
use crate::core::shared::data::Entity;
use crate::core::shared::repositories::query::Paged;
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee, ManyView, PageView, PaginationView};
use crate::models::shared::views::{DataWrapperView, LinkView};
use crate::models::shared::views::entities::{EntityView, LinksEntityView};

pub trait CanBeManyView<T: Serialize + Clone> {
    fn to_many_view(
        &self,
        ctx: &Context,
        ontology: String,
        other_link: HashMap<String, String>,
    ) -> ManyView<T>;
}

impl<T: Serialize + Clone> CanBeManyView<T> for Paged<T> {
    fn to_many_view(&self, ctx: &Context, ontology: String, other_link: HashMap<String, String>) -> ManyView<T> {
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
            .chain(vec!["page[number]=0".to_string(), format!("page[size]={}", self.meta.page.size)])
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
            .chain(vec![format!("page[number]={}", max(if self.meta.page.number > 0 { self.meta.page.number - 1 } else { 0 }, 0)), format!("page[size]={}", self.meta.page.size)])
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

        ManyView {
            data: self.data.clone(),
            meta: Some(
                PaginationView {
                    total_pages: self.meta.total_pages.clone(),
                    total_records: self.meta.total_records.clone(),
                    page: PageView {
                        number: self.meta.page.number.clone(),
                        size: self.meta.page.size.clone(),
                    },
                }
            ),
            links: Some(links),
        }
    }
}

pub fn from_states_to_entity_view<DATA, VIEW>(
    entity: Entity<DATA, String>,
    ontology: String,
    context: &Context,
) -> EntityView<VIEW>
where
    VIEW: Serialize + Clone,
    DATA: Clone + CanBeView<VIEW> + CanGetTypee,
{
    let entity_id = entity.entity_id.as_str();

    let external_url = context.meta
        .get("externalUrl")
        .map(|urlref| urlref.clone())
        .unwrap_or("unknown".to_string());

    let link_event = (
        "events".to_string(), format!("{external_url}/{ontology}/{entity_id}/events")
    );
    let link_self = (
        "self".to_string(), format!("{external_url}/{ontology}/{entity_id}")
    );

    EntityView {
        r#type: entity.data.get_type(),
        id: entity_id.to_string(),
        attributes: entity.data.to_view(),
        links: Some(LinksEntityView {
            links: HashMap::from([link_event, link_self])
        }),
    }
}

pub fn from_states_to_view<DATA, VIEW>(
    entity: Entity<DATA, String>,
    ontology: String,
    context: &Context,
) -> DataWrapperView<EntityView<VIEW>>
where
    VIEW: Serialize + Clone,
    DATA: Clone + CanBeView<VIEW> + CanGetTypee,
{
    DataWrapperView {
        data: from_states_to_entity_view(entity, ontology, context)
    }
}