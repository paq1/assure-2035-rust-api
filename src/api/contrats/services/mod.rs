use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::clients::data::states::ClientStates;
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::contrats::services::ContratService;
use crate::core::contrats::services::facteur_pays_repo::FacteurPaysRepo;
use crate::core::contrats::services::facteur_vehicle_repo::FacteurVehicleRepo;
use crate::core::contrats::services::formule_service::FormuleService;
use crate::core::shared::context::Context;
use crate::core::shared::id_generator::IdGenerator;
use crate::core::shared::repositories::entities::RepositoryEntity;
use crate::core::shared::repositories::events::RepositoryEvents;
use crate::models::contrats::commands::DeleteContratCommand;
use crate::models::contrats::shared::CurrencyValue;
use crate::models::shared::errors::{Error, ResultErr};

pub mod formule_service_impl;
pub mod formule_repo_mock;
pub mod facteur_vehicle_repo_mock;
pub mod facteur_pays_repo_mock;

pub struct ContratsServiceImpl {
    pub store: Arc<dyn RepositoryEntity<ContratStates, String>>,
    pub journal: Arc<dyn RepositoryEvents<ContratEvents, String>>,
    pub formule_service: Arc<Mutex<dyn FormuleService>>,
    pub facteur_vehicle_repo: Arc<Mutex<dyn FacteurVehicleRepo>>,
    pub facteur_pays_repo: Arc<Mutex<dyn FacteurPaysRepo>>,
    pub store_personne: Arc<dyn RepositoryEntity<ClientStates, String>>,
}

#[async_trait]
impl ContratService for ContratsServiceImpl {
    async fn delete_contrat(&self, _command: DeleteContratCommand, _id: String, _ctx: Context) -> ResultErr<String> {
        todo!()
    }

    async fn get_client_country_code(&self, id_client: &String) -> ResultErr<String> {
        let maybe_client = self.store_personne
            .fetch_one(id_client.clone()).await?;
        match maybe_client {
            Some(entity_client) => {
                match entity_client.data {
                    ClientStates::ClientActif(client) =>
                        client.data.address
                            .ok_or(Error::Simple("impossible de souscrire un contrat sans adresse".to_string()))
                            .map(|address| address.country),
                    _ => Err(Error::Simple("le client n'est pas actif, il ne peut pas souscrire a un contract".to_string()))
                }
            }
            None => Err(Error::Simple("pas de client trouvée".to_string()))
        }
    }

    async fn get_formule_from_code(&self, code: &String) -> ResultErr<CurrencyValue> {
        self.formule_service.lock().await.get_formule_from_code(code).await
    }

    async fn get_facteur_vehicule_from_code(&self, code: &String) -> ResultErr<f32> {
        self.facteur_vehicle_repo.lock().await.fetch_one(code).await
    }

    async fn get_facteur_pays_from_code(&self, code: &String) -> ResultErr<f32> {
        self.facteur_pays_repo.lock().await.fetch_one(code).await
    }
}

impl IdGenerator for ContratsServiceImpl {
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
