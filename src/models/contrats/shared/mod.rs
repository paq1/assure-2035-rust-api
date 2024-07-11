use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct ContractData {
    pub holder: String,
    pub product: String,
    pub formula: String,
    pub vehicle: Vehicle,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Vehicle {
    pub plate: String,
    pub brand: String,
    #[serde(rename = "insuredValue")]
    pub insured_value: InsuredValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct InsuredValue {
    value: f32,
    currency: String
}
