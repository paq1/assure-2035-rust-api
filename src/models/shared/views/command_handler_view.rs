use serde::{Deserialize, Serialize};
use crate::core::shared::data::EntityEvent;
use crate::models::shared::jsonapi::CanBeView;
use crate::models::shared::views::DataWrapperView;

pub fn from_output_command_handler_to_view<DATA, VIEW>(
    self_url: String,
    event: EntityEvent<DATA, String>,
    ontology: String,
) -> DataWrapperView<ApiView<VIEW>>
where
    VIEW: Serialize + Clone,
    DATA: Clone + CanBeView<VIEW>,
{
    let type_urn_event = format!("org:example:insurance:client:event"); // fixme
    let event_id = event.event_id;
    let state_id = event.entity_id;
    let urn_state_type = "org:example:insurance:client";
    DataWrapperView {
        data: ApiView {
            r#type: type_urn_event.to_string(),
            id: event_id.clone(),
            attributes: AttributesEvent {
                attributes: event.data.to_view(),
            },
            relationships: Relationships {
                entity: DataWrapper {
                    data: DataRS {
                        r#type: urn_state_type.to_string(),
                        id: state_id.clone(),
                    }
                }
            },
            links: Link {
                selfevent: Some(format!("{self_url}/{ontology}/{state_id}/events/{event_id}"))
            },
        }
    }

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiView<T>
where
    T: Serialize + Clone,
{
    #[serde(rename = "type")]
    pub r#type: String,
    pub id: String,
    pub attributes: AttributesEvent<T>,
    pub relationships: Relationships,
    pub links: Link,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributesEvent<T>
where
    T: Serialize + Clone,
{
    #[serde(flatten)]
    pub attributes: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Relationships {
    #[serde(rename = "_entity")]
    pub entity: DataWrapper,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataWrapper {
    pub data: DataRS,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataRS {
    #[serde(rename = "type")]
    pub r#type: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    #[serde(rename = "self")]
    pub r#selfevent: Option<String>,
}

