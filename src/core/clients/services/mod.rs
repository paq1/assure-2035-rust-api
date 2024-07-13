use async_trait::async_trait;

use crate::core::shared::context::Context;
use crate::models::clients::commands::*;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait ClientService {
    async fn delete_client(&self, command: DisableClientCommand, id: String, ctx: Context) -> ResultErr<String>;
}