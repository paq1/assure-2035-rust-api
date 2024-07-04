use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientStates {
    Client (ClientData)
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ClientData {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: DateTime<Utc>
}
impl ClientStates {
    pub fn data(&self) -> ClientData {
        match self {
            ClientStates::Client(client_data) => client_data.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientEvents {
    Created {
        by: String,
        #[serde(with = "ts_seconds")]
        at: DateTime<Utc>,
        first_name: String,
        last_name: String,
        birth_date: DateTime<Utc>,
    },
    Updated(UpdatedEvent),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    pub name: String,
}