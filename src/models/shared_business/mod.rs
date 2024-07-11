use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Adresse {
    pub street: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub locality: String,
    pub country: String,
}