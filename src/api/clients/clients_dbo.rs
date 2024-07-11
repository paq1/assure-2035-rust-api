use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::clients::shared::{ClientData, DisableReason};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientDboState {
    ClientDbo {
        #[serde(rename = "_kind")]
        kind: String,

        #[serde(flatten)]
        data: ClientData,
    },
    ClientDisableDbo {
        #[serde(rename = "_kind")]
        kind: String,

        #[serde(flatten)]
        data: ClientData,
        
        reason: DisableReason,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientDboEvent {
    Created(ClientCreatedDbo),
    Updated(ClientUpdatedDbo),
    Disable(ClientDisabledDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: ClientData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: ClientData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientDisabledDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub reason: DisableReason,
}

