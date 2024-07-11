use std::sync::Arc;
use async_trait::async_trait;
use futures::lock::Mutex;
use crate::core::contrats::services::formule_repo::FormuleRepo;
use crate::core::contrats::services::formule_service::FormuleService;
use crate::models::contrats::shared::CurrencyValue;
use crate::models::shared::errors::ResultErr;
pub struct FormuleServiceImpl {
    pub formule_repo: Arc<Mutex<Box<dyn FormuleRepo>>>
}

#[async_trait]
impl FormuleService for FormuleServiceImpl {
    async fn get_all(&self) -> Vec<(String, CurrencyValue)> {
        self.formule_repo.lock().await.fetch_all().await
    }

    async fn can_get_formule(&self, name: &String) -> ResultErr<CurrencyValue> {
        self.formule_repo.lock().await.fetch_one(name).await
    }
}