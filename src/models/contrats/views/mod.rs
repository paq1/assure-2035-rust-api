use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::contrats::shared::ContractData;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ContratView {
    #[serde(flatten)]
    pub data: ContractData
}
