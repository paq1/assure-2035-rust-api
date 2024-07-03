use chrono::{DateTime, Utc};

pub struct Context {
    pub subject: String,
    pub now: DateTime<Utc>
}