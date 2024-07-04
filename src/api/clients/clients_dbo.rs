use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientDboState {
    ClientDbo {
        #[serde(rename = "firstName")]
        first_name: String,
        #[serde(rename = "lastName")]
        last_name: String,
        #[serde(rename = "birthDate")]
        birth_date: DateTime<Utc>,
    }
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
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub name: String,
}
