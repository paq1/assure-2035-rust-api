pub mod daos;
pub mod token;
pub mod query;
pub mod cache;
pub mod helpers;

#[derive(Clone, Debug)]
pub struct OwnUrl {
    pub url: String,
}

impl OwnUrl {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}