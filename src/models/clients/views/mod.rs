use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::clients::shared::ClientData;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ClientView {
    #[serde(flatten)]
    pub data: ClientData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum ClientViewState {
    #[serde(rename = "actif")]
    Client (ClientViewActif)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientViewActif {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: DateTime<Utc>
}


#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ClientUpdatedView {
    #[serde(flatten)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum ClientViewEvent {
    #[serde(rename = "created")]
    Created(ClientView),
    #[serde(rename = "updated")]
    Updated(ClientUpdatedView)
}
