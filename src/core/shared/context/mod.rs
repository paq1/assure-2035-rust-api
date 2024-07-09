use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Context {
    pub subject: String,
    pub now: DateTime<Utc>,
    pub meta: HashMap<String, String>,
}