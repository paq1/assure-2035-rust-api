use actix_web::HttpRequest;
use crate::api::shared::helpers::context::CanDecoreFromHttpRequest;
use crate::api::shared::token::jwt_claims::JwtClaims;
use crate::core::shared::context::Context;
use crate::core::shared::token::TokenService;
use crate::models::shared::errors::{Error, ErrorHttpCustom, ResultErr};

pub async fn authenticated<T: TokenService>(
    req: &HttpRequest,
    jwt_token_service: &T,
) -> ResultErr<Context> {
    let maybe_authorization = req.headers().get("Authorization");
    match maybe_authorization {
        Some(bearer_header_value) => {
            let bearer_str = bearer_header_value
                .to_str()
                .map_err(|err| Error::Http(ErrorHttpCustom::new(
                    err.to_string(),
                    "00TOKPA".to_string(),
                    vec![],
                    None,
                ))
                )?;

            let jwt = *bearer_str
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap_or(&"");

            let ctx: ResultErr<Context> = jwt_token_service
                .decode::<JwtClaims>(jwt).await
                .map(|claims| claims.into())
                .map_err(|err| {
                    println!("err: {err:?}");
                    err
                });
            ctx.map(|ct| ct.decore_with_http_header(req))
        }
        _ => Err(
            Error::Http(
                ErrorHttpCustom::new(
                    "Unauthorized, pas de token d'authentification".to_string(),
                    "00MTOKE".to_string(),
                    vec![],
                    Some(401),
                )
            )
        )
    }
}

