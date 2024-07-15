use std::sync::Arc;
use futures::lock::Mutex;
use crate::api::clients::clients_dbo::{ClientDboEvent, ClientDboState};
use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_dao::{ClientsEventMongoDAO, ClientsMongoDAO};
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use crate::api::clients::services::ClientsServiceImpl;
use crate::api::shared::daos::dbos::{EntityDBO, EventDBO};
use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::clients::services::ClientService;
use crate::core::shared::daos::DAO;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::core::shared::repositories::entities::RepositoryEntity;
use crate::core::shared::repositories::events::RepositoryEvents;
use crate::models::clients::commands::ClientsCommands;
use crate::core::clients::command_handler::create_handler::CreateClientHandler;
use crate::core::clients::command_handler::disable_handler::DisableClientHandler;
use crate::core::clients::command_handler::update_handler::UpdateClientHandler;
use crate::core::clients::reducer::ClientReducer;

pub struct ClientComponent {
    pub store: Arc<dyn RepositoryEntity<ClientStates, String>>,
    pub journal: Arc<dyn RepositoryEvents<ClientEvents, String>>,
    pub service: Arc<dyn ClientService>,
    pub engine: Arc<Engine<ClientStates, ClientsCommands, ClientEvents>>,
}

impl ClientComponent {

    pub async fn new() -> Self {

        let dbname = "seedassure2035mongo";

        let dao_store_client: Arc<Mutex<dyn DAO<EntityDBO<ClientDboState, String>, String>>> =
            Arc::new(Mutex::new(ClientsMongoDAO::new(dbname, "clients_store_actix").await));
        let dao_journal_client: Arc<Mutex<dyn DAO<EventDBO<ClientDboEvent, String>, String>>> =
            Arc::new(Mutex::new(ClientsEventMongoDAO::new(dbname, "clients_journal_actix").await));

        // repo
        let store: Arc<dyn RepositoryEntity<ClientStates, String>> = Arc::new(
            ClientsMongoRepository {
                dao: Arc::clone(&dao_store_client)
            }
        );
        let journal: Arc<dyn RepositoryEvents<ClientEvents, String>> = Arc::new(
            ClientsEventMongoRepository {
                dao: Arc::clone(&dao_journal_client)
            }
        );
        // services
        let service: Arc<dyn ClientService> = Arc::new(
            ClientsServiceImpl {
                store: Arc::clone(&store),
                journal: Arc::clone(&journal),
            }
        );

        let engine: Arc<Engine<ClientStates, ClientsCommands, ClientEvents>> = Arc::new(Engine {
            handlers: vec![
                CommandHandler::Create(Box::new(CreateClientHandler {})),
                CommandHandler::Update(Box::new(UpdateClientHandler {})),
                CommandHandler::Update(Box::new(DisableClientHandler {})),
            ],
            reducer: ClientReducer::new().underlying,
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