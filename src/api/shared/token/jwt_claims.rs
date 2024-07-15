use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::core::shared::context::Context;

impl From<JwtClaims> for Context {
    fn from(value: JwtClaims) -> Self {
        Self {
            subject: value.sub,
            name: value.name,
            given_name: value.given_name,
            family_name: value.family_name,
            email: value.email,
            now: Utc::now(),
            meta: HashMap::new(),
            filters: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims {
    pub sub: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}