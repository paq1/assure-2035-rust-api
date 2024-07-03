use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityDBO<DATA, ID> {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id_mongo: Option<ObjectId>,
    pub version: Option<u32>,
    #[serde(rename = "id")]
    pub entity_id: ID,
    pub data: DATA,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventDBO<DATA, ID> {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id_mongo: Option<ObjectId>,
    pub version: Option<u32>,
    pub entity_id: ID,
    #[serde(rename = "id")]
    pub event_id: ID,
    pub data: DATA,
}
