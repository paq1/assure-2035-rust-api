use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use jsonwebtoken::{Algorithm, decode, decode_header, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::api::shared::cache::CacheAsync;
use crate::core::shared::token::TokenService;
use crate::models::shared::errors::{Error, ResultErr};

pub struct JwtRSATokenService {
    pub cache: Arc<CacheAsync>
}

impl JwtRSATokenService {
    pub fn new(cache: Arc<CacheAsync>) -> Self {
        Self {
            cache
        }
    }
}

#[async_trait]
impl TokenService for JwtRSATokenService {

    async fn decode<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> ResultErr<CLAIMS> {
        let header = decode_header(token).map_err(|err| {
            let message = err.to_string();
            Error::Simple(format!("decode header token : {message}"))
        })?;

        let kid = header.kid.ok_or(Error::Simple("jwt invalid, pas de kid dans l'entete".to_string()))?;
        let maybe_data = self.cache.get(kid.clone()).await;

        let jwk = match maybe_data {
            Some(data) => {
                let jwk = serde_json::from_str::<JWK>(data.as_str())
                    .map_err(|err| Error::Simple(err.to_string()))?;
                jwk
            },
            None => {
                // todo authback call http
                let response = JWK {
                    n: "nn".to_string(),
                    e: "ee".to_string()
                };
                let stringify = serde_json::to_string(&response)
                    .map_err(|err| Error::Simple(err.to_string()))?;
                self.cache.upsert(kid, stringify).await;
                response
            }
        };


        let decoding_key = DecodingKey::from_rsa_components(
            jwk.n.as_str(),
            jwk.e.as_str()
        ).map_err(|err| Error::Simple({
            let error_message = err.to_string();
            format!("decoding key : {error_message}")
        }))?;


        decode::<CLAIMS>(token, &decoding_key, &Validation::new(Algorithm::RS256))
            .map(|token_data| token_data.claims)
            .map_err(|err| {
                let message = err.to_string();
                Error::Simple(format!("decode token : {message}"))
            })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWK {
    n: String,
    e: String
}