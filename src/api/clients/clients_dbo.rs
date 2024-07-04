use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientDboState {
    ClientDbo { name: String }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientDboEvent {
    Created(ClientCreatedDbo),
    Updated(ClientUpdatedDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub name: String,
}
