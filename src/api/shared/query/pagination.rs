use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

#[derive(IntoParams, Serialize, Deserialize, Debug)]
pub struct HttpPaginationQuery {
    #[serde(rename = "page[number]")]
    pub number: Option<usize>,
    #[serde(rename = "page[size]")]
    pub size: Option<usize>,
}