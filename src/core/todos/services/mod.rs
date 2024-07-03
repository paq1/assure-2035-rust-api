use async_trait::async_trait;
use crate::core::shared::context::Context;
use crate::models::shared::errors::ResultErr;
use crate::models::todos::commands::*;

#[async_trait]
pub trait TodosService {
    async fn create_todo(&self, command: CreateTodoCommand, ctx: Context) -> ResultErr<String>;
    async fn update_todo(&self, command: UpdateTodoCommand, id: String, ctx: Context) -> ResultErr<String>;
    async fn delete_todo(&self, command: DeleteTodoCommand, id: String, ctx: Context) -> ResultErr<String>;
}