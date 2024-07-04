use async_trait::async_trait;
use crate::core::shared::context::Context;
use crate::models::shared::errors::ResultErr;
use crate::models::clients::commands::*;

#[async_trait]
pub trait ClientService {
    async fn delete_client(&self, command: DeleteClientCommand, id: String, ctx: Context) -> ResultErr<String>;
}