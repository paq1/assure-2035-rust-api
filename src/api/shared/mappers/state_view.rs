use std::collections::HashMap;
use serde::Serialize;
use crate::core::shared::context::Context;
use crate::core::shared::data::Entity;
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};
use crate::models::shared::views::DataWrapperView;
use crate::models::shared::views::entities::{EntityView, LinksEntityView};

pub fn from_states_to_entity_view<DATA, VIEW>(
    entity: Entity<DATA, String>,
    ontology: String,
    context: &Context
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
        })
    }

}

pub fn from_states_to_view<DATA, VIEW>(
    entity: Entity<DATA, String>,
    ontology: String,
    context: &Context
) -> DataWrapperView<EntityView<VIEW>>
where
    VIEW: Serialize + Clone,
    DATA: Clone + CanBeView<VIEW> + CanGetTypee,
{
    DataWrapperView {
        data: from_states_to_entity_view(entity, ontology, context)
    }
}