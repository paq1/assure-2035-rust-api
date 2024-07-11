use async_trait::async_trait;
use crate::core::shared::context::Context;
use crate::models::shared::errors::ResultErr;
use crate::models::contrats::commands::*;

#[async_trait]
pub trait ContratService {
    async fn delete_contrat(&self, command: DeleteContratCommand, id: String, ctx: Context) -> ResultErr<String>;
}