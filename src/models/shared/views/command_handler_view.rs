use serde::{Deserialize, Serialize};

use crate::models::shared::views::LinkView;

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
    pub links: LinkView,
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
    pub links: RelatedLinks,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelatedLinks {
    pub related: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataRS {
    #[serde(rename = "type")]
    pub r#type: String,
    pub id: String,
}


