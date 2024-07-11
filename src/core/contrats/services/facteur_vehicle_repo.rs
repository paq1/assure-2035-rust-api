use async_trait::async_trait;

use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait FacteurVehicleRepo: Send + Sync {
    async fn fetch_all(&self) -> Vec<(String, f32)>;
    async fn fetch_one(&self, code: &String) -> ResultErr<f32>;
}