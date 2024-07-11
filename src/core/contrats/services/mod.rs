pub mod formule_service;
pub mod formule_repo;

use async_trait::async_trait;
use crate::core::shared::context::Context;
use crate::models::shared::errors::ResultErr;
use crate::models::contrats::commands::*;

#[async_trait]
pub trait ContratService: Send + Sync {
    async fn delete_contrat(&self, command: DeleteContratCommand, id: String, ctx: Context) -> ResultErr<String>;
}