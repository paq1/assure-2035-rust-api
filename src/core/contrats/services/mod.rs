use async_trait::async_trait;

use crate::models::contrats::shared::{ContractData, CurrencyValue};
use crate::models::shared::errors::{Error, ErrorHttpCustom, ResultErr};

pub mod formule_service;
pub mod formule_repo;
pub mod facteur_vehicle_repo;
pub mod facteur_pays_repo;

#[async_trait]
pub trait ContratService: Send + Sync {
    async fn calcul_premium(&self, command: &ContractData) -> ResultErr<CurrencyValue> {
        let id_client = command.holder.clone();

        let country_code = self.get_client_country_code(&id_client).await.map_err(|_| {
            Error::Http(
                ErrorHttpCustom {
                    title: "todo".to_string(),
                    code: "todo".to_string(),
                    causes: vec![],
                    status: Some(404),
                }
            )
        })?;

        let facteur_pays = self.get_facteur_pays_from_code(&country_code).await?;

        let marque_vehicle = command.vehicle.brand.clone();

        let facteur_vehicle = self.get_facteur_vehicule_from_code(&marque_vehicle).await?;

        let formule_code = command.formula.clone();

        let prime_base = self.get_formule_from_code(&formule_code).await?;

        let value = prime_base.value * facteur_pays * facteur_vehicle;
        let value_sanitize = value.round();

        Ok(CurrencyValue {
            value: value_sanitize,
            currency: command.vehicle.insured_value.currency.clone(),
        })
    }

    async fn get_client_country_code(&self, id_client: &String) -> ResultErr<String>;

    async fn get_formule_from_code(&self, code: &String) -> ResultErr<CurrencyValue>;
    async fn get_facteur_vehicule_from_code(&self, code: &String) -> ResultErr<f32>;
    async fn get_facteur_pays_from_code(&self, code: &String) -> ResultErr<f32>;
}