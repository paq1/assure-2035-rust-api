use async_trait::async_trait;

use crate::core::contrats::services::facteur_pays_repo::FacteurPaysRepo;
use crate::models::shared::errors::ResultErr;

pub struct FacteurPaysRepoMock {}

#[async_trait]
impl FacteurPaysRepo for FacteurPaysRepoMock {
    async fn fetch_all(&self) -> Vec<(String, f32)> {
        vec![
            ("CH".to_string(), 0.8),
            ("LU".to_string(), 1.2),
            ("Autres".to_string(), 1.0),
        ]
    }

    async fn fetch_one(&self, code: &String) -> ResultErr<f32> {
        self.fetch_all().await
            .into_iter()
            .find(|(current_code, _)| *current_code == *code)
            .map(|(_, value)| Ok(value))
            .unwrap_or(Ok(1.0))
    }
}