use async_trait::async_trait;

use crate::core::shared::context::Context;
use crate::models::clients::commands::*;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait ClientService: Send + Sync {
    async fn delete_client(&self, command: DisableClientCommand, id: String, ctx: Context) -> ResultErr<String>;
}