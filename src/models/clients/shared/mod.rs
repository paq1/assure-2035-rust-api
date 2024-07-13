use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::shared_business::Adresse;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(tag = "type")]
pub enum PhoneNumber {
    Simple(String),
    Phone(Phone),
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Phone {
    pub country: String,
    #[serde(rename = "nationalNumber")]
    pub national_number: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct ClientData {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: NaiveDate,

    #[serde(rename = "drivingLicenseDate", skip_serializing_if = "Option::is_none")]
    pub driving_license_date: Option<NaiveDate>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Adresse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub enum DisableReason {
    #[serde(rename = "gdpr")]
    GDPR,
    #[serde(rename = "death")]
    DEATH,
}
