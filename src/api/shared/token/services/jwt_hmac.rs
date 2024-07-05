use std::fmt::Debug;

use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::shared::token::TokenService;
use crate::models::shared::errors::{Error, ResultErr};

pub struct JwtHMACTokenService {
    secret: String
}

impl JwtHMACTokenService {
    pub fn new(secret: String) -> Self {
        Self {
            secret
        }
    }
}

#[async_trait]
impl TokenService for JwtHMACTokenService {

    async fn decode<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> ResultErr<CLAIMS> {
        decode::<CLAIMS>(token, &DecodingKey::from_secret(self.secret.as_bytes()), &Validation::default())
            .map(|token_data| token_data.claims)
            .map_err(|err| Error::Simple(err.to_string()))
    }

    // async fn decode_public<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> ResultErr<CLAIMS> {
    //     let header = decode_header(token).map_err(|err| {
    //         let message = err.to_string();
    //         Error::Simple(format!("decode header token : {message}"))
    //     })?;
    //
    //     let decoding_key = DecodingKey::from_rsa_components(
    //         "nqKVTe2BFaPhPMDJpJpqTyNLmpvQOc8ugu4aJQb-r75jbQPaPhl3zTLyeIAFinPDofy9N7ocYZAudHDtdSxonM3AelmYhhDHPxpVzpcXhxEL7_jaLr1u4X1r_nW6vdcsCwnP0Zy4WcP3F2Ls13wbSuRbtX3opyk98-TI3KSceoESNsOMX0vR5sYMOTk52yjA5Tp1lqEol04NX-Loy9yDsi0kiYJOBFWgojt8FDpTVswx887Pc5z_PpaKArugiVphJk02mzqC9kFq2Pdb605fZzunyWTeB0868uTmPrDqXClx6dGgsnC55zmLG8X8sfPxoxGFHw5UUwwmS14z64QSYw",
    //         "AQAB"
    //     ).map_err(|err| Error::Simple({
    //         let error_message = err.to_string();
    //         format!("decoding key : {error_message}")
    //     }))?;
    //
    //
    //     decode::<CLAIMS>(token, &decoding_key, &Validation::new(Algorithm::RS256))
    //         .map(|token_data| token_data.claims)
    //         .map_err(|err| {
    //             let message = err.to_string();
    //             Error::Simple(format!("decode token : {message}"))
    //         })
    // }
}
