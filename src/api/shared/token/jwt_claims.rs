use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::core::shared::context::Context;

impl From<JwtClaims> for Context {
    fn from(value: JwtClaims) -> Self {
        Self {
            subject: value.name,
            now: Utc::now()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims {
    pub name: String
}