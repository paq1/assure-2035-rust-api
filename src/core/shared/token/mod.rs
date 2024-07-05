use std::fmt::Debug;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait TokenService {
    async fn decode<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> ResultErr<CLAIMS>;
}