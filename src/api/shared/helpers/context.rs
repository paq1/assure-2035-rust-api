use std::collections::HashMap;

use actix_web::HttpRequest;

use crate::api::shared::helpers::header_value::CanSanitizeHeader;
use crate::core::shared::context::Context;
use crate::models::shared::errors::ResultErr;

impl CanDecoreFromHttpRequest for Context {
    fn decore_with_http_header(&self, req: &HttpRequest) -> ResultErr<Self> {
        let maybe_proto = req.headers()
            .get("X-Forwarded-Proto")
            .map(|header_value| header_value.clone().sanitize_header("X-Forwarded-Proto".to_string()))
            .map(|x| x.map(|x| Some(x)).unwrap_or(None))
            .flatten();

        let maybe_host = req.headers()
            .get("X-Forwarded-Host")
            .map(|header_value| header_value.clone().sanitize_header("X-Forwarded-Host".to_string()))
            .map(|x| x.map(|x| Some(x)).unwrap_or(None))
            .flatten();

        let maybe_prefix = req.headers()
            .get("X-Forwarded-Prefix")
            .map(|header_value| header_value.clone().sanitize_header("X-Forwarded-Prefix".to_string()))
            .map(|x| x.map(|x| Some(x)).unwrap_or(None))
            .flatten();

        let maybe_external_url = match (maybe_proto.clone(), maybe_host.clone(), maybe_prefix.clone()) {
            (Some(proto), Some(host), Some(prefix)) =>
                Some(format!("{}:{}{}", proto.1, host.1, prefix.1)),
            (Some(proto), Some(host), None) =>
                Some(format!("{}:{}", proto.1, host.1)),
            _ => None
        }.map(|val| ("externalUrl".to_string(), val));

        let meta = vec![maybe_proto, maybe_host, maybe_prefix, maybe_external_url]
            .iter()
            .fold(HashMap::new(), |acc, current| {
                match current {
                    Some((key, value)) => acc
                        .into_iter()
                        .chain(HashMap::from([(key.clone(), value.clone())]))
                        .collect::<HashMap<String, String>>(),

                    None => acc
                }
            });

        Ok(
            Context {
                meta,
                ..self.clone()
            }
        )
    }
}

pub trait CanDecoreFromHttpRequest: Sized {
    fn decore_with_http_header(&self, req: &HttpRequest) -> ResultErr<Self>;
}