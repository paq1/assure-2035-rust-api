use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::models::shared::errors::{Error, ResultErr};

impl<T> CanToHttpResponse<T> for ResultErr<T>
where
    T: Serialize + DeserializeOwned
{

    fn to_created(&self) -> HttpResponse {
        match self {
            Ok(k) => HttpResponse::Created().json(k),
            Err(err) => {
                match err {
                    Error::Http(http_error) => {
                        match http_error.status {
                            Some(400) => HttpResponse::BadRequest().json(err),
                            Some(404) => HttpResponse::NotFound().json(err),
                            _ => HttpResponse::InternalServerError().json(err)
                        }
                    },
                    Error::Simple(_) => HttpResponse::InternalServerError().json(err)
                }
            }
        }
    }

    fn to_ok(&self) -> HttpResponse {
        match self {
            Ok(k) => HttpResponse::Ok().json(k),
            Err(err) => {
                match err {
                    Error::Http(http_error) => {
                        match http_error.status {
                            Some(400) => HttpResponse::BadRequest().json(err),
                            Some(404) => HttpResponse::NotFound().json(err),
                            _ => HttpResponse::InternalServerError().json(err)
                        }
                    },
                    Error::Simple(_) => HttpResponse::InternalServerError().json(err)
                }
            }
        }
    }
}

pub trait CanToHttpResponse<T> {
    fn to_created(&self) -> HttpResponse;
    fn to_ok(&self) -> HttpResponse;
}