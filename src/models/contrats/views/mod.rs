use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ContratView {
    #[schema(example = "Roger")]
    pub name: String,
}
