use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::core::shared::data::EntityEvent;
use crate::core::shared::repositories::query::Paged;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Many<T>
where
    T: Serialize + Clone,
{
    #[schema(example = "[]")]
    pub data: Vec<T>,
    pub meta: Option<Pagination>,
}

impl<T: Serialize + Clone> Many<T> {
    pub fn new(paged: Paged<T>) -> Self {
        Self {
            data: paged.data,
            meta: Some(
                Pagination {
                    total_pages: paged.meta.total_pages,
                    number: paged.meta.number,
                    size: paged.meta.size
                }
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Pagination {
    #[serde(rename = "totalPages")]
    pub total_pages: usize,
    pub number: usize,
    pub size: usize,
}

pub fn from_t_to_view<E>(
    self_url: String,
    event: EntityEvent<E, String>,
    ontology: String,
) -> ApiView<E>
where
    E: Serialize + Clone,
{
    let type_urn_event = "org:example:insurance:client:event"; // fixme
    let event_id = event.event_id;
    let type_event = "created"; // fixme
    let state_id = event.entity_id;
    let urn_state_type = "org:example:insurance:client";

    ApiView {
        r#type: type_urn_event.to_string(),
        id: event_id.clone(),
        attributes: AttributesEvent {
            attributes: event.data,
            event_type: type_event.to_string(),
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
    pub attributes: T,
    #[serde(rename = "eventType")]
    pub event_type: String,
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

