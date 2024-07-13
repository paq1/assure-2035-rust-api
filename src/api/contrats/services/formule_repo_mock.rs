use async_trait::async_trait;
use crate::core::contrats::services::formule_repo::FormuleRepo;
use crate::models::contrats::shared::CurrencyValue;
use crate::models::shared::errors::{Error, ResultErr};

pub struct FormuleRepoMock {}

#[async_trait]
impl FormuleRepo for FormuleRepoMock {
    async fn fetch_all(&self) -> Vec<(String, CurrencyValue)> {
        vec![
            ("bronze".to_string(), CurrencyValue { value: 600.0, currency: "EUR".to_string() }),
            ("silver".to_string(), CurrencyValue { value: 900.0, currency: "EUR".to_string() }),
            ("gold".to_string(), CurrencyValue { value: 1500.0, currency: "EUR".to_string() }),
        ]
    }

    async fn fetch_one(&self, code: &String) -> ResultErr<CurrencyValue> {
        self.fetch_all().await
            .into_iter()
            .find(|(current_code, _)| *current_code == *code)
            .map(|(_, value)| value)
            .ok_or(Error::Simple("not found".to_string()))
    }
}