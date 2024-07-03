use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity<S, REF>
where
    S: Clone,
    REF: Clone
{
    pub entity_id: REF,
    pub data: S,
    pub version: Option<u32>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntityEvent<E, REF> {
    pub entity_id: REF,
    pub data: E,
    pub event_id: REF
}
