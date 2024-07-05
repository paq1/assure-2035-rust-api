use std::fmt::Debug;
use async_trait::async_trait;
use jsonwebtoken::{Algorithm, decode, decode_header, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::core::shared::token::TokenService;
use crate::models::shared::errors::{Error, ResultErr};

pub struct JwtRSATokenService {}

impl JwtRSATokenService {
    // pub fn new() -> Self {
    //     Self {}
    // }
}

#[async_trait]
impl TokenService for JwtRSATokenService {

    async fn decode<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> ResultErr<CLAIMS> {
        let _header = decode_header(token).map_err(|err| {
            let message = err.to_string();
            Error::Simple(format!("decode header token : {message}"))
        })?;

        let decoding_key = DecodingKey::from_rsa_components(
            "xxx",
            "AQAB"
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
