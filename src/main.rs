use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use futures::lock::Mutex;
use moka::future::Cache;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::shared::token::services::jwt_hmac::JwtHMACTokenService;
use crate::api::swagger::ApiDoc;
use api::clients::routes::read_routes::{fetch_many_client, fetch_one_client};
use crate::api::clients::services::ClientsServiceImpl;
use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_dao::{ClientsEventMongoDAO, ClientsMongoDAO};
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use api::clients::routes::write_routes::{insert_one_client, update_one_client};
use crate::api::clients::routes::read_routes::fetch_events_client;
use crate::api::contrats::contrats_event_mongo_repository::ContratsEventMongoRepository;
use crate::api::contrats::contrats_mongo_dao::{ContratsEventMongoDAO, ContratsMongoDAO};
use crate::api::contrats::contrats_mongo_repository::ContratsMongoRepository;
use crate::api::contrats::routes::read_routes::{fetch_events_contrat, fetch_many_contrat, fetch_one_contrat};
use crate::api::contrats::routes::write_routes::{insert_one_contrat, update_one_contrat};
use crate::api::contrats::services::ContratsServiceImpl;
use crate::api::shared::cache::CacheAsync;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::core::clients::command_handler::command_handler_impl::{CreateClientHandler, UpdateClientHandler};
use crate::core::clients::data::{ClientEvents, ClientStates};
use crate::core::clients::reducer::ClientReducer;
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::contrats::reducer::ContratReducer;
use crate::models::shared::errors::StandardHttpError;
use crate::models::clients::commands::ClientsCommands;
use crate::models::contrats::commands::ContratsCommands;
use crate::core::contrats::command_handler::command_handler_impl::{CreateContratHandler, UpdateContratHandler};

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let dbname = "seedassure2035mongo";

    let cache = Arc::new(CacheAsync { underlying: Cache::new(10_000) });


    // todo delete exemple
    // let locked_cache = Arc::clone(&cache);
    // let result_compute = match Arc::clone(&cache)
    //     .get("1234".to_string()).await {
    //     Some(r) => {
    //         r
    //     }
    //     None => {
    //         println!("compute result");
    //         let value = "onfsdodnfosndklnfnsldnflnsdflknls".to_string();
    //         Arc::clone(&cache).clone().upsert("1234".to_string(), value.clone()).await;
    //         value
    //     }
    // };
    // let result_compute2 = match Arc::clone(&cache).clone()
    //     .get("1234".to_string()).await {
    //     Some(result) => {
    //         result
    //     }
    //     None => {
    //         println!("compute result");
    //         let value = "onfsdodnfosndklnfnsldnflnsdflknls".to_string();
    //         Arc::clone(&cache).clone().upsert("1234".to_string(), value.clone()).await;
    //         value
    //     }
    // };
    //
    // println!("result : {result_compute} et {result_compute2}");


    // client ontology
    let store_clients: Arc<Mutex<ClientsMongoRepository>> = Arc::new(
        Mutex::new(
            ClientsMongoRepository {
                dao: ClientsMongoDAO::new(dbname.to_string(), "clients_store_actix".to_string()).await
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
    let clients_service: Arc<Mutex<ClientsServiceImpl<ClientsMongoRepository, ClientsEventMongoRepository>>> = Arc::new(
        Mutex::new(
            ClientsServiceImpl {
                store: Arc::clone(&store_clients),
                journal: Arc::clone(&journal_clients),
            }
        )
    );
    let engine_client: Arc<Mutex<Engine<ClientStates, ClientsCommands, ClientEvents, ClientsMongoRepository, ClientsEventMongoRepository>>> = Arc::new(Mutex::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateClientHandler {})),
            CommandHandler::Update(Box::new(UpdateClientHandler {})),
        ],
        reducer: ClientReducer::new().underlying,
        store: Arc::clone(&store_clients),
        journal: Arc::clone(&journal_clients),
    }));

    // contrat ontology
    let store_contrats: Arc<Mutex<ContratsMongoRepository>> = Arc::new(
        Mutex::new(
            ContratsMongoRepository {
                dao: ContratsMongoDAO::new(dbname.to_string(), "contrats_store_actix".to_string()).await
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
    let contrats_service: Arc<Mutex<ContratsServiceImpl<ContratsMongoRepository, ContratsEventMongoRepository>>> = Arc::new(
        Mutex::new(
            ContratsServiceImpl {
                store: Arc::clone(&store_contrats),
                journal: Arc::clone(&journal_contrats),
            }
        )
    );
    let engine_contrat: Arc<Mutex<Engine<ContratStates, ContratsCommands, ContratEvents, ContratsMongoRepository, ContratsEventMongoRepository>>> = Arc::new(Mutex::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateContratHandler {})),
            CommandHandler::Update(Box::new(UpdateContratHandler {})),
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
        let jwt_rsa_token_service = JwtRSATokenService::new(Arc::clone(&cache));


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
            .service(fetch_many_client)
            .service(fetch_events_client)
            .service(insert_one_client)
            .service(update_one_client)
            // contrats routes
            .service(fetch_one_contrat)
            .service(fetch_many_contrat)
            .service(fetch_events_contrat)
            .service(insert_one_contrat)
            .service(update_one_contrat)
    })
        .workers(2)
        .bind((api_address, api_port))?
        .run()
        .await
}