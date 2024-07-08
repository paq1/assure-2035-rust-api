use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct ClientData {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub enum DisableReason {
    #[serde(rename = "gdpr")]
    GDPR,
    #[serde(rename = "death")]
    DEATH
}
