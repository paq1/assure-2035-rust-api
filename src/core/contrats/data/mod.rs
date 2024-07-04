use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ContratStates {
    Contrat { name: String }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContratEvents {
    Created {
        by: String,
        #[serde(with = "ts_seconds")]
        at: DateTime<Utc>,
        name: String
    },
    Updated (UpdatedEvent)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatedEvent {
    pub by: String,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
    pub name: String
}