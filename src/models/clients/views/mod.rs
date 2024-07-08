use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::clients::shared::{ClientData, DisableReason};

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ClientView {
    #[serde(flatten)]
    pub data: ClientData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum ClientViewState {
    #[serde(rename = "actif")]
    Client (ClientViewActif),
    ClientDisable(ClientViewDisable),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientViewActif {
    #[serde(flatten)]
    pub data: ClientData
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientViewDisable {
    #[serde(flatten)]
    pub data: ClientData
}


#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ClientUpdatedView {
    #[serde(flatten)]
    pub data: ClientData
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ClientDisabledView {
    pub reason: DisableReason
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum ClientViewEvent {
    #[serde(rename = "created")]
    Created(ClientView),
    #[serde(rename = "updated")]
    Updated(ClientUpdatedView),
    #[serde(rename = "disabled")]
    Disabled(ClientDisabledView),
}
