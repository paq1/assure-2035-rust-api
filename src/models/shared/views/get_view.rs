use std::collections::HashMap;

use serde::Serialize;

use crate::core::shared::data::Entity;
use crate::models::shared::jsonapi::{CanBeView, CanGetTypee};
use crate::models::shared::views::DataWrapperView;
use crate::models::shared::views::entities::{EntityView, LinksEntityView};

pub fn from_states_to_view<DATA, VIEW>(
    self_url: String,
    entity: Entity<DATA, String>,
    ontology: String,
) -> DataWrapperView<EntityView<VIEW>>
where
    VIEW: Serialize + Clone,
    DATA: Clone + CanBeView<VIEW> + CanGetTypee,
{
    let entity_id = entity.entity_id.as_str();

    DataWrapperView {
        data: EntityView {
            r#type: entity.data.get_type(),
            id: entity_id.to_string(),
            attributes: entity.data.to_view(),
            links: Some(LinksEntityView {
                events: format!("{self_url}/{ontology}/{entity_id}/events"),
                self_entity: format!("{self_url}/{ontology}/{entity_id}"),
                links: HashMap::new()
            })
        }
    }

}