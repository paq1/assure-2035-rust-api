use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TodoDboState {
    TodoDbo { name: String }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TodoDboEvent {
    TodoCreatedDbo {
        by: String,
        at: DateTime<Utc>,
        name: String
    },
    Updated(TodoUpdatedDbo)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoUpdatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    pub name: String
}
