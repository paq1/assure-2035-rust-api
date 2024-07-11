use async_trait::async_trait;

use crate::core::contrats::services::facteur_vehicle_repo::FacteurVehicleRepo;
use crate::models::shared::errors::{Error, ResultErr};

pub struct FacteurVehicleRepoMock {}

#[async_trait]
impl FacteurVehicleRepo for FacteurVehicleRepoMock {
    async fn fetch_all(&self) -> Vec<(String, f32)> {
        vec![
            ("Renault".to_string(), 0.5),
            ("Fiat".to_string(), 0.6),
            ("Ford".to_string(), 0.7),
            ("Nissan".to_string(), 0.75),
            ("Peugeot".to_string(), 0.8),
            ("Volkswagen".to_string(), 1.1),
            ("Audi".to_string(), 1.25),
            ("BMW".to_string(), 1.25),
            ("Mercedes".to_string(), 1.25),
            ("Ferrari".to_string(), 2.3),
            ("Bugatti".to_string(), 2.6),
            ("Tesla".to_string(), 5.0),
            ("Autres".to_string(), 1.0),
        ]
    }

    async fn fetch_one(&self, code: &String) -> ResultErr<f32> {
        self.fetch_all().await
            .into_iter()
            .find(|(current_code, _)| *current_code == *code)
            .map(|(_, value)| value)
            .ok_or(Error::Simple("not found".to_string()))
    }
}