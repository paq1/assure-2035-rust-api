use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct EntityView<T>
where
    T: Serialize + Clone
{
    pub r#type: String,
    pub id: String,
    pub attributes: T,
    pub links: LinksEntity,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct LinksEntity {
    pub events: String,
    #[serde(rename = "self")]
    pub self_entity: String,
}
