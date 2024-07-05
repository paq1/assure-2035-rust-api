use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::clients::shared::ClientData;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ClientView {
    #[serde(flatten)]
    pub data: ClientData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ClientUpdatedView {
    #[serde(flatten)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[serde(tag = "eventType")]
pub enum ClientViewEvent {
    #[serde(rename = "created")]
    Created(ClientView),
    #[serde(rename = "updated")]
    Updated(ClientUpdatedView)
}
