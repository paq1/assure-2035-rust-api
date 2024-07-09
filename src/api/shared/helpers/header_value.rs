use actix_web::http::header::HeaderValue;
use crate::models::shared::errors::{Error, ResultErr};

pub trait CanSanitizeHeader {
    fn sanitize_header(&self, name: String) -> ResultErr<(String, String)>;
}

impl CanSanitizeHeader for HeaderValue {
    fn sanitize_header(&self, name: String) -> ResultErr<(String, String)> {
        self.to_str()
            .map_err(|err| Error::Simple(err.to_string()))
            .map(|res| (name, res.to_string()))
    }
}

