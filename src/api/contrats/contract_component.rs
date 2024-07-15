use std::sync::Arc;

use futures::lock::Mutex;

use crate::api::contrats::contrats_event_mongo_repository::ContratsEventMongoRepository;
use crate::api::contrats::contrats_mongo_dao::{ContratsEventMongoDAO, ContratsMongoDAO};
use crate::api::contrats::contrats_mongo_repository::ContratsMongoRepository;
use crate::api::contrats::services::ContratsServiceImpl;
use crate::api::contrats::services::facteur_pays_repo_mock::FacteurPaysRepoMock;
use crate::api::contrats::services::facteur_vehicle_repo_mock::FacteurVehicleRepoMock;
use crate::api::contrats::services::formule_repo_mock::FormuleRepoMock;
use crate::api::contrats::services::formule_service_impl::FormuleServiceImpl;
use crate::core::clients::data::states::ClientStates;
use crate::core::contrats::command_handler::approve_command_handler::ApproveContractHandler;
use crate::core::contrats::command_handler::command_handler_impl::{CreateContratHandler, UpdateContratHandler};
use crate::core::contrats::command_handler::reject_command_handler::RejectContractHandler;
use crate::core::contrats::command_handler::terminate_command_handler::TerminateContractHandler;
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::contrats::reducer::ContratReducer;
use crate::core::contrats::services::ContratService;
use crate::core::contrats::services::facteur_pays_repo::FacteurPaysRepo;
use crate::core::contrats::services::facteur_vehicle_repo::FacteurVehicleRepo;
use crate::core::contrats::services::formule_repo::FormuleRepo;
use crate::core::contrats::services::formule_service::FormuleService;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::core::shared::repositories::entities::RepositoryEntity;
use crate::core::shared::repositories::events::RepositoryEvents;
use crate::models::contrats::commands::ContratsCommands;

pub struct ContractComponent {
    pub store: Arc<dyn RepositoryEntity<ContratStates, String>>,
    pub journal: Arc<dyn RepositoryEvents<ContratEvents, String>>,
    pub service: Arc<dyn ContratService>,
    pub engine: Arc<Engine<ContratStates, ContratsCommands, ContratEvents>>,
}

impl ContractComponent {

    pub async fn new(client_store: Arc<dyn RepositoryEntity<ClientStates, String>>) -> Self {

        let dbname = "seedassure2035mongo";

        let store: Arc<dyn RepositoryEntity<ContratStates, String>> = Arc::new(
            ContratsMongoRepository {
                dao: Arc::new(Mutex::new(ContratsMongoDAO::new(dbname, "contrats_store_actix").await))
            }
        );
        let journal: Arc<dyn RepositoryEvents<ContratEvents, String>> = Arc::new(
            ContratsEventMongoRepository {
                dao: ContratsEventMongoDAO::new(dbname, "contrats_journal_actix").await
            }
        );

        let formume_repo: Arc<dyn FormuleRepo> = Arc::new(FormuleRepoMock {});

        let formule_service: Arc<dyn FormuleService> = Arc::new(FormuleServiceImpl { formule_repo: Arc::clone(&formume_repo) });

        let facteur_vehicle_repo: Arc<dyn FacteurVehicleRepo> = Arc::new(
            FacteurVehicleRepoMock {}
        );

        let facteur_pays_repo: Arc<dyn FacteurPaysRepo> = Arc::new(
            FacteurPaysRepoMock {}
        );

        let service: Arc<dyn ContratService> = Arc::new(
            ContratsServiceImpl {
                formule_service: Arc::clone(&formule_service),
                store_personne: Arc::clone(&client_store),
                facteur_pays_repo: Arc::clone(&facteur_pays_repo),
                facteur_vehicle_repo: Arc::clone(&facteur_vehicle_repo),
            }
        );

        let engine: Arc<Engine<ContratStates, ContratsCommands, ContratEvents>> = Arc::new(Engine {
            handlers: vec![
                CommandHandler::Create(Box::new(CreateContratHandler { contract_service: Arc::clone(&service) })),
                CommandHandler::Update(Box::new(UpdateContratHandler { contract_service: Arc::clone(&service) })),
                CommandHandler::Update(Box::new(ApproveContractHandler {})),
                CommandHandler::Update(Box::new(RejectContractHandler {})),
                CommandHandler::Update(Box::new(TerminateContractHandler {})),
            ],
            reducer: ContratReducer::new().underlying,
            store: Arc::clone(&store),
            journal: Arc::clone(&journal),
        });

        Self {
            store,
            journal,
            service,
            engine,
        }
    }
}