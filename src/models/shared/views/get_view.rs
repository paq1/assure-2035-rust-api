use serde::Serialize;

use crate::core::shared::data::Entity;
use crate::models::shared::jsonapi::CanBeView;
use crate::models::shared::views::DataWrapperView;
use crate::models::shared::views::entities::{EntityView, LinksEntity};

pub fn from_states_to_view<DATA, VIEW>(
    self_url: String,
    entity: Entity<DATA, String>,
    ontology: String,
) -> DataWrapperView<EntityView<VIEW>>
where
    VIEW: Serialize + Clone,
    DATA: Serialize + Clone + CanBeView<VIEW>,
{
    let entity_id = entity.entity_id.as_str();

    DataWrapperView {
        data: EntityView {
            r#type: format!("org:example:insurance:{ontology}"),
            id: entity_id.to_string(),
            attributes: entity.data.to_view(),
            links: LinksEntity {
                events: format!("{self_url}/{ontology}/{entity_id}/events"),
                self_entity: format!("{self_url}/{ontology}/{entity_id}"),
            }
        }
    }

}