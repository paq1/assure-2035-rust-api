use async_trait::async_trait;

use crate::core::shared::context::Context;
use crate::models::contrats::commands::*;
use crate::models::contrats::shared::CurrencyValue;
use crate::models::shared::errors::ResultErr;

pub mod formule_service;
pub mod formule_repo;
pub mod facteur_vehicle_repo;
pub mod facteur_pays_repo;

#[async_trait]
pub trait ContratService: Send + Sync {
    async fn delete_contrat(&self, command: DeleteContratCommand, id: String, ctx: Context) -> ResultErr<String>;
    async fn calcul_premium(&self, command: CreateContratCommand) -> ResultErr<CurrencyValue> {
        let id_client = command.data.holder;

        let country_code = self.get_client_country_code(&id_client).await?;

        let facteur_pays = self.get_facteur_pays_from_code(&country_code).await?;

        let marque_vehicle = command.data.vehicle.brand;

        let facteur_vehicle = self.get_facteur_vehicule_from_code(&marque_vehicle).await?;

        let formule_code = command.data.formula;

        let prime_base = self.get_formule_from_code(&formule_code).await?;

        let value = prime_base.value * facteur_pays * facteur_vehicle;
        let value_sanitize = value.round();

        Ok(CurrencyValue {
            value: value_sanitize,
            currency: command.data.vehicle.insured_value.currency
        })

    }

    async fn get_client_country_code(&self, id_client: &String) -> ResultErr<String>;

    async fn get_formule_from_code(&self, code: &String) -> ResultErr<CurrencyValue>;
    async fn get_facteur_vehicule_from_code(&self, code: &String) -> ResultErr<f32>;
    async fn get_facteur_pays_from_code(&self, code: &String) -> ResultErr<f32>;
}