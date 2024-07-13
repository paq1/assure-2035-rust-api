use std::collections::HashMap;
use serde::Serialize;
use crate::core::shared::context::Context;
use crate::core::shared::data::EntityEvent;
use crate::models::shared::jsonapi::CanBeView;
use crate::models::shared::views::command_handler_view::{ApiView, AttributesEvent, DataRS, DataWrapper, RelatedLinks, Relationships};
use crate::models::shared::views::{DataWrapperView, LinkView};

// fixme renommer ca sert a la vue des evenements en vrai
pub fn from_output_command_handler_to_view<DATA, VIEW>(
    event: EntityEvent<DATA, String>,
    ontology: String,
    complete_ontology: String,
    context: &Context,
) -> DataWrapperView<ApiView<VIEW>>
where
    VIEW: Serialize + Clone,
    DATA: Clone + CanBeView<VIEW>,
{
    let type_urn_event = format!("{complete_ontology}:event");
    let event_id = event.event_id;
    let state_id = event.entity_id;

    let external_url = context.meta
        .get("externalUrl")
        .map(|urlref| urlref.clone())
        .unwrap_or("unknown".to_string());

    DataWrapperView {
        data: ApiView {
            r#type: type_urn_event,
            id: event_id.clone(),
            attributes: AttributesEvent {
                attributes: event.data.to_view(),
            },
            relationships: Relationships {
                entity: DataWrapper {
                    data: DataRS {
                        r#type: complete_ontology,
                        id: state_id.clone(),
                    },
                    links: RelatedLinks {
                        related: format!("{external_url}/{ontology}/{state_id}")
                    }
                }
            },
            links: LinkView {
                links: HashMap::from([
                    (
                        "self".to_string(),
                        format!("{external_url}/{ontology}/{state_id}/events/{event_id}")
                    )
                ]),
            },
        }
    }
}