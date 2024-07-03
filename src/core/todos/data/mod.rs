use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum TodoStates {
    Todo { name: String }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TodoEvents {
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