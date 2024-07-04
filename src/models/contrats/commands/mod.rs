use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ContratsCommands {
    Create (CreateContratCommand),
    Update (UpdateContratCommand),
    Delete (DeleteContratCommand)
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateContratCommand {
    #[schema(example = "input")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateContratCommand {
    #[schema(example = "input")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct DeleteContratCommand;
