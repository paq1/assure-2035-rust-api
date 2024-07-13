use async_trait::async_trait;

use crate::models::contrats::shared::CurrencyValue;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait FormuleRepo: Send + Sync {
    async fn fetch_all(&self) -> Vec<(String, CurrencyValue)>;
    async fn fetch_one(&self, code: &String) -> ResultErr<CurrencyValue>;
}