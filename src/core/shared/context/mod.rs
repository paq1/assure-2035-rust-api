use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Context {
    pub subject: String,
    pub name: String,
    // pub given_name: String, pas besoins ici selon les specs :)
    // pub family_name: String, pas besoins ici selon les specs :)
    pub email: String,
    pub now: DateTime<Utc>,
    pub meta: HashMap<String, String>,
    pub filters: HashMap<String, String>,
}

impl Context {
    pub fn empty() -> Self {
        Self {
            subject: "usr:unknown".to_string(),
            name: "".to_string(),
            email: "".to_string(),
            now: Utc::now(),
            meta: HashMap::new(),
            filters: HashMap::new(),
        }
    }

    pub fn clone_with_filter(&self, filters: HashMap<String, String>) -> Self {
        Self {
            filters,
            ..self.clone()
        }
    }
}