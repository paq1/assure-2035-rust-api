use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ClientDboState {
    ClientDbo { name: String }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ClientDboEvent {
    ClientCreatedDbo {
        by: String,
        at: DateTime<Utc>,
        name: String
    },
    Updated(ClientUpdatedDbo)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub name: String
}
