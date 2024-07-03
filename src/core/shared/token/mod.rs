use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::models::shared::errors::ResultErr;

pub trait TokenService {
    fn decode<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> ResultErr<CLAIMS>;
}