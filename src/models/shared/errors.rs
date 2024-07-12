use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type ResultErr<DATA> = Result<DATA, Error>;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum Error {
    Http(ErrorHttpCustom),
    Simple(String),
    // Problem(Problem)
}


#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct StandardHttpError {
    pub not_found: ErrorHttpCustom,
    pub internal_server_error: ErrorHttpCustom,
    pub unauthorized: ErrorHttpCustom,
}

impl StandardHttpError {
    pub fn new() -> Self {
        Self {
            not_found: ErrorHttpCustom::new("ressource not found".to_string(), "00NOTFO".to_string(), vec![], Some(404)),
            internal_server_error: ErrorHttpCustom::new("wip".to_string(), "00INTER".to_string(), vec![], Some(500)),
            unauthorized: ErrorHttpCustom::new("wip".to_string(), "00UNAUT".to_string(), vec![], Some(401)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ErrorHttpCustom {
    #[schema(example = "titre")]
    pub title: String,
    #[schema(example = "00EXAMPLE")]
    pub code: String,
    #[schema(example = "[]")]
    pub causes: Vec<Problem>,
    #[schema(example = "200")]
    pub status: Option<u16>,
}

impl ErrorHttpCustom {
    pub fn new(title: String, code: String, problems: Vec<Problem>, status: Option<u16>) -> Self {
        Self {
            title,
            code,
            causes: problems,
            status,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Problem {
    #[schema(example = "titre")]
    pub title: String,
    #[schema(example = "description")]
    pub description: String,
}