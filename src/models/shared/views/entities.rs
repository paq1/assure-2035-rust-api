use std::collections::HashMap;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<LinksEntityView>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct LinksEntityView {
    pub events: String,
    #[serde(rename = "self")]
    pub self_entity: String,
    #[serde(flatten)]
    pub links: HashMap<String, String>,
}
