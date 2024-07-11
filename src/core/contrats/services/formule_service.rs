use async_trait::async_trait;

use crate::models::contrats::shared::CurrencyValue;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait FormuleService: Send + Sync {

    async fn get_all(&self) -> Vec<(String, CurrencyValue)>;

    async fn can_get_formule(&self, name: &String) -> ResultErr<CurrencyValue>;
}