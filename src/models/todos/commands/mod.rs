use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum TodoCommands {
    Create (CreateTodoCommand),
    Update (UpdateTodoCommand),
    Delete (DeleteTodoCommand)
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateTodoCommand {
    #[schema(example = "input")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateTodoCommand {
    #[schema(example = "input")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DeleteTodoCommand;
