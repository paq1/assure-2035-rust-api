use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::shared::errors::{Error, ResultErr};

pub enum HttpKindResponse {
    Created,
    Ok
}

impl HttpKindResponse {
    pub fn to_http_response<T>(&self, data: &T) -> HttpResponse
    where
        T: Serialize + DeserializeOwned
    {
        match self {
            HttpKindResponse::Created => HttpResponse::Created().json(data),
            HttpKindResponse::Ok => HttpResponse::Ok().json(data),
        }
    }
}

impl<T> CanToHttpResponse<T> for ResultErr<T>
where
    T: Serialize + DeserializeOwned
{

    fn to_http_response_with_error_mapping(&self, http_status: HttpKindResponse) -> HttpResponse {
        match self {
            Ok(k) => http_status.to_http_response(k),
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
    fn to_http_response_with_error_mapping(&self, http_status: HttpKindResponse) -> HttpResponse;
}