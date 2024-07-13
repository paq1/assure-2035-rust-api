use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use futures::lock::Mutex;
use moka::future::Cache;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::clients::routes::read_routes::{fetch_many_client, fetch_one_client};
use api::clients::routes::write_routes::{insert_one_client, update_one_client};

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
use crate::api::contrats::routes::write_routes::{approve_one_contrat, insert_one_contrat, update_one_contrat};
use crate::api::contrats::services::ContratsServiceImpl;
use crate::api::contrats::services::facteur_pays_repo_mock::FacteurPaysRepoMock;
use crate::api::contrats::services::facteur_vehicle_repo_mock::FacteurVehicleRepoMock;
use crate::api::contrats::services::formule_repo_mock::FormuleRepoMock;
use crate::api::contrats::services::formule_service_impl::FormuleServiceImpl;
use crate::api::shared::cache::CacheAsync;
use crate::api::shared::token::services::jwt_hmac::JwtHMACTokenService;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::api::swagger::ApiDoc;
use crate::core::clients::command_handler::command_handler_impl::{CreateClientHandler, DisableClientHandler, UpdateClientHandler};
use crate::core::clients::data::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::clients::reducer::ClientReducer;
use crate::core::contrats::command_handler::command_handler_impl::{CreateContratHandler, UpdateContratHandler};
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::contrats::reducer::ContratReducer;
use crate::core::contrats::services::ContratService;
use crate::core::contrats::services::facteur_pays_repo::FacteurPaysRepo;
use crate::core::contrats::services::facteur_vehicle_repo::FacteurVehicleRepo;
use crate::core::contrats::services::formule_repo::FormuleRepo;
use crate::core::contrats::services::formule_service::FormuleService;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::models::clients::commands::ClientsCommands;
use crate::models::contrats::commands::ContratsCommands;
use crate::models::shared::errors::StandardHttpError;
use crate::core::contrats::command_handler::approve_command_handler::ApproveContractHandler;
use crate::core::shared::repositories::entities::RepositoryEntity;

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
    let store_clients: Arc<Mutex<dyn RepositoryEntity<ClientStates, String>>> = Arc::new(
        Mutex::new(
            ClientsMongoRepository {
                dao: Arc::new(Mutex::new(ClientsMongoDAO::new(dbname.to_string(), "clients_store_actix".to_string()).await))
            }
        )
    );

    let journal_clients: Arc<Mutex<ClientsEventMongoRepository>> = Arc::new(
        Mutex::new(
            ClientsEventMongoRepository {
                dao: ClientsEventMongoDAO::new(dbname.to_string(), "clients_journal_actix".to_string()).await
            }
        )
    );
    let clients_service: Arc<Mutex<ClientsServiceImpl<ClientsEventMongoRepository>>> = Arc::new(
        Mutex::new(
            ClientsServiceImpl {
                store: Arc::clone(&store_clients),
                journal: Arc::clone(&journal_clients),
            }
        )
    );
    let engine_client: Arc<Mutex<Engine<ClientStates, ClientsCommands, ClientEvents, ClientsEventMongoRepository>>> = Arc::new(Mutex::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateClientHandler {})),
            CommandHandler::Update(Box::new(UpdateClientHandler {})),
            CommandHandler::Update(Box::new(DisableClientHandler {})),
        ],
        reducer: ClientReducer::new().underlying,
        store: Arc::clone(&store_clients),
        journal: Arc::clone(&journal_clients),
    }));

    // contrat ontology
    let store_contrats: Arc<Mutex<dyn RepositoryEntity<ContratStates, String>>> = Arc::new(
        Mutex::new(
            ContratsMongoRepository {
                dao: Arc::new(Mutex::new(ContratsMongoDAO::new(dbname.to_string(), "contrats_store_actix".to_string()).await))
            }
        )
    );

    let journal_contrats: Arc<Mutex<ContratsEventMongoRepository>> = Arc::new(
        Mutex::new(
            ContratsEventMongoRepository {
                dao: ContratsEventMongoDAO::new(dbname.to_string(), "contrats_journal_actix".to_string()).await
            }
        )
    );

    let formume_repo: Arc<Mutex<dyn FormuleRepo>> = Arc::new(Mutex::new(FormuleRepoMock {}));

    let formule_service: Arc<Mutex<dyn FormuleService>> = Arc::new(Mutex::new(FormuleServiceImpl { formule_repo: Arc::clone(&formume_repo) }));

    let facteur_vehicle_repo: Arc<Mutex<dyn FacteurVehicleRepo>> = Arc::new(
        Mutex::new(
            FacteurVehicleRepoMock {}
        )
    );

    let facteur_pays_repo: Arc<Mutex<dyn FacteurPaysRepo>> = Arc::new(
        Mutex::new(
            FacteurPaysRepoMock {}
        )
    );

    let contrats_service: Arc<Mutex<dyn ContratService>> = Arc::new(
        Mutex::new(
            ContratsServiceImpl {
                store: Arc::clone(&store_contrats),
                journal: Arc::clone(&journal_contrats),
                formule_service: Arc::clone(&formule_service),
                store_personne: Arc::clone(&store_clients),
                facteur_pays_repo: Arc::clone(&facteur_pays_repo),
                facteur_vehicle_repo: Arc::clone(&facteur_vehicle_repo),
            }
        )
    );

    let engine_contrat: Arc<Mutex<Engine<ContratStates, ContratsCommands, ContratEvents, ContratsEventMongoRepository>>> = Arc::new(Mutex::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateContratHandler { contract_service: Arc::clone(&contrats_service) })),
            CommandHandler::Update(Box::new(UpdateContratHandler {})),
            CommandHandler::Update(Box::new(ApproveContractHandler {})),
        ],
        reducer: ContratReducer::new().underlying,
        store: Arc::clone(&store_contrats),
        journal: Arc::clone(&journal_contrats),
    }));

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
            .service(update_one_contrat)
    })
        .workers(2)
        .bind((api_address, api_port))?
        .run()
        .await
}