use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use futures::lock::Mutex;
use moka::future::Cache;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::clients::routes::read_routes::{fetch_many_client, fetch_one_client};
use api::clients::routes::write_routes::{insert_one_client, update_one_client};

use crate::api::clients::clients_dbo::{ClientDboEvent, ClientDboState};
use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_dao::{ClientsEventMongoDAO, ClientsMongoDAO};
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use crate::api::clients::routes::read_routes::{fetch_events_client, fetch_one_client_event};
use crate::api::clients::routes::write_routes::disable_one_client;
use crate::api::clients::services::ClientsServiceImpl;
use crate::api::contrats::contrats_event_mongo_repository::ContratsEventMongoRepository;
use crate::api::contrats::contrats_mongo_dao::{ContratsEventMongoDAO, ContratsMongoDAO};
use crate::api::contrats::contrats_mongo_repository::ContratsMongoRepository;
use crate::api::contrats::routes::read_routes::{fetch_events_contrat, fetch_many_contrat, fetch_one_contract_event, fetch_one_contrat};
use crate::api::contrats::routes::write_routes::{approve_one_contrat, insert_one_contrat, reject_one_contrat, update_one_contrat};
use crate::api::contrats::services::ContratsServiceImpl;
use crate::api::contrats::services::facteur_pays_repo_mock::FacteurPaysRepoMock;
use crate::api::contrats::services::facteur_vehicle_repo_mock::FacteurVehicleRepoMock;
use crate::api::contrats::services::formule_repo_mock::FormuleRepoMock;
use crate::api::contrats::services::formule_service_impl::FormuleServiceImpl;
use crate::api::shared::cache::CacheAsync;
use crate::api::shared::daos::dbos::{EntityDBO, EventDBO};
use crate::api::shared::token::services::jwt_hmac::JwtHMACTokenService;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::api::swagger::ApiDoc;
use crate::core::clients::command_handler::create_handler::CreateClientHandler;
use crate::core::clients::command_handler::disable_handler::DisableClientHandler;
use crate::core::clients::command_handler::update_handler::UpdateClientHandler;
use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::clients::reducer::ClientReducer;
use crate::core::clients::services::ClientService;
use crate::core::contrats::command_handler::approve_command_handler::ApproveContractHandler;
use crate::core::contrats::command_handler::reject_command_handler::RejectContractHandler;
use crate::core::contrats::command_handler::command_handler_impl::{CreateContratHandler, UpdateContratHandler};
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::contrats::reducer::ContratReducer;
use crate::core::contrats::services::ContratService;
use crate::core::contrats::services::facteur_pays_repo::FacteurPaysRepo;
use crate::core::contrats::services::facteur_vehicle_repo::FacteurVehicleRepo;
use crate::core::contrats::services::formule_repo::FormuleRepo;
use crate::core::contrats::services::formule_service::FormuleService;
use crate::core::shared::daos::DAO;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::core::shared::repositories::entities::RepositoryEntity;
use crate::core::shared::repositories::events::RepositoryEvents;
use crate::models::clients::commands::ClientsCommands;
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::StandardHttpError;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let dbname = "seedassure2035mongo";
    let auth_back_url = std::env::var("AUTH_BACK_URL").unwrap_or("http://localhost:9001".to_string());

    let cache = Arc::new(CacheAsync { underlying: Cache::new(10_000) });
    let http_client = Arc::new(reqwest::Client::new());


    // client ontology
    // dao
    let dao_store_client: Arc<Mutex<dyn DAO<EntityDBO<ClientDboState, String>, String>>> =
        Arc::new(Mutex::new(ClientsMongoDAO::new(dbname, "clients_store_actix").await));
    let dao_journal_client: Arc<Mutex<dyn DAO<EventDBO<ClientDboEvent, String>, String>>> =
        Arc::new(Mutex::new(ClientsEventMongoDAO::new(dbname, "clients_journal_actix").await));
    // repo
    let store_clients: Arc<dyn RepositoryEntity<ClientStates, String>> = Arc::new(
        ClientsMongoRepository {
            dao: Arc::clone(&dao_store_client)
        }
    );
    let journal_clients: Arc<dyn RepositoryEvents<ClientEvents, String>> = Arc::new(
        ClientsEventMongoRepository {
            dao: Arc::clone(&dao_journal_client)
        }
    );
    // services
    let clients_service: Arc<dyn ClientService> = Arc::new(
        ClientsServiceImpl {
            store: Arc::clone(&store_clients),
            journal: Arc::clone(&journal_clients),
        }
    );
    // event sourcing
    let engine_client: Arc<Engine<ClientStates, ClientsCommands, ClientEvents>> = Arc::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateClientHandler {})),
            CommandHandler::Update(Box::new(UpdateClientHandler {})),
            CommandHandler::Update(Box::new(DisableClientHandler {})),
        ],
        reducer: ClientReducer::new().underlying,
        store: Arc::clone(&store_clients),
        journal: Arc::clone(&journal_clients),
    });

    // contrat ontology
    let store_contrats: Arc<dyn RepositoryEntity<ContratStates, String>> = Arc::new(
        ContratsMongoRepository {
            dao: Arc::new(Mutex::new(ContratsMongoDAO::new(dbname, "contrats_store_actix").await))
        }
    );
    let journal_contrats: Arc<dyn RepositoryEvents<ContratEvents, String>> = Arc::new(
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

    let contrats_service: Arc<dyn ContratService> = Arc::new(
        ContratsServiceImpl {
            store: Arc::clone(&store_contrats),
            journal: Arc::clone(&journal_contrats),
            formule_service: Arc::clone(&formule_service),
            store_personne: Arc::clone(&store_clients),
            facteur_pays_repo: Arc::clone(&facteur_pays_repo),
            facteur_vehicle_repo: Arc::clone(&facteur_vehicle_repo),
        }
    );

    let engine_contrat: Arc<Engine<ContratStates, ContratsCommands, ContratEvents>> = Arc::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateContratHandler { contract_service: Arc::clone(&contrats_service) })),
            CommandHandler::Update(Box::new(UpdateContratHandler { contract_service: Arc::clone(&contrats_service) })),
            CommandHandler::Update(Box::new(ApproveContractHandler {})),
            CommandHandler::Update(Box::new(RejectContractHandler {})),
        ],
        reducer: ContratReducer::new().underlying,
        store: Arc::clone(&store_contrats),
        journal: Arc::clone(&journal_contrats),
    });

    let openapi = ApiDoc::openapi();
    let api_address = std::env::var("API_ADDRESS").unwrap();
    let api_port = std::env::var("API_PORT").unwrap().parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        let standard_http_error = StandardHttpError::new();
        let jwt_token_service = JwtHMACTokenService::new("test".to_string());
        let jwt_rsa_token_service = JwtRSATokenService::new(Arc::clone(&cache), Arc::clone(&http_client), auth_back_url.clone());


        App::new()
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .app_data(web::Data::new(jwt_rsa_token_service))
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(jwt_token_service))
            // clients services
            .app_data(web::Data::new(Arc::clone(&engine_client)))
            .app_data(
                web::Data::new(Arc::clone(&store_clients))
            )
            .app_data(
                web::Data::new(Arc::clone(&journal_clients))
            )
            .app_data(
                web::Data::new(Arc::clone(&clients_service))
            )
            // contrats services
            .app_data(web::Data::new(Arc::clone(&engine_contrat)))
            .app_data(
                web::Data::new(Arc::clone(&store_contrats))
            )
            .app_data(
                web::Data::new(Arc::clone(&journal_contrats))
            )
            .app_data(
                web::Data::new(Arc::clone(&contrats_service))
            )
            // client routes
            .service(fetch_one_client)
            .service(fetch_one_client_event)
            .service(fetch_many_client)
            .service(fetch_events_client)
            .service(insert_one_client)
            .service(update_one_client)
            .service(disable_one_client)
            // contrats routes
            .service(fetch_one_contract_event)
            .service(fetch_one_contrat)
            .service(fetch_many_contrat)
            .service(fetch_events_contrat)
            .service(insert_one_contrat)
            .service(approve_one_contrat)
            .service(reject_one_contrat)
            .service(update_one_contrat)
    })
        .workers(2)
        .bind((api_address, api_port))?
        .run()
        .await
}