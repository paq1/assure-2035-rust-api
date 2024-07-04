use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ContratDboState {
    ContratDbo { name: String }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ContratDboEvent {
    ContratCreatedDbo {
        by: String,
        at: DateTime<Utc>,
        name: String
    },
    Updated(ContratUpdatedDbo)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContratUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub name: String
}
