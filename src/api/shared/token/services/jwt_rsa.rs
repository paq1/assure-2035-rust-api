use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use jsonwebtoken::{Algorithm, decode, decode_header, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::api::shared::cache::CacheAsync;
use crate::core::shared::token::TokenService;
use crate::models::shared::errors::{Error, ResultErr};

pub struct JwtRSATokenService {
    pub cache: Arc<CacheAsync>,
    pub http_client: Arc<Client>,
    pub auth_back_url: String
}

impl JwtRSATokenService {
    pub fn new(cache: Arc<CacheAsync>, http_client: Arc<Client>, auth_back_url: String) -> Self {
        Self {
            cache,
            http_client,
            auth_back_url
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
                Ok(jwk)
            },
            None => {
                let url = format!("{}/v1/jwks/{kid}/public", self.auth_back_url);
                let response = self.http_client
                    .get(url)
                    .send()
                    .await.map_err(|err| Error::Simple(err.to_string()))?;
                if response.status() == 200 {
                    let jwk = response.json::<JWK>().await.map_err(|err| Error::Simple(err.to_string()))?;
                    let stringify = serde_json::to_string(&jwk)
                        .map_err(|err| Error::Simple(err.to_string()))?;
                    self.cache.upsert(kid, stringify).await;
                    Ok(jwk)
                } else {
                    Err(Error::Simple("erreur lors du call authbacku".to_string()))
                }
            }
        }?;

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
