use std::sync::Arc;

use async_trait::async_trait;

use crate::core::contrats::services::formule_repo::FormuleRepo;
use crate::core::contrats::services::formule_service::FormuleService;
use crate::models::contrats::shared::CurrencyValue;
use crate::models::shared::errors::ResultErr;

pub struct FormuleServiceImpl {
    pub formule_repo: Arc<dyn FormuleRepo>,
}

#[async_trait]
impl FormuleService for FormuleServiceImpl {

    async fn get_formule_from_code(&self, name: &String) -> ResultErr<CurrencyValue> {
        self.formule_repo.fetch_one(name).await
    }
}