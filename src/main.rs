use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use futures::lock::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::shared::token::JwtTokenService;
use crate::api::swagger::ApiDoc;
use api::clients::routes::read_routes::{fetch_many, fetch_one};
use crate::api::clients::services::ClientsServiceImpl;
use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_dao::{ClientsEventMongoDAO, ClientsMongoDAO};
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use api::clients::routes::write_routes::{insert_one, update_one};
use crate::api::clients::routes::read_routes::fetch_events;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::core::clients::command_handler::command_handler_impl::{CreateClientHandler, UpdateClientHandler};
use crate::core::clients::data::{ClientEvents, ClientStates};
use crate::core::clients::reducer::ClientReducer;
use crate::models::shared::errors::StandardHttpError;
use crate::models::clients::commands::ClientsCommands;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let store: Arc<Mutex<ClientsMongoRepository>> = Arc::new(
        Mutex::new(
            ClientsMongoRepository {
                dao: ClientsMongoDAO::new("seedassure2035mongo".to_string(), "clients_store_actix".to_string()).await
            }
        )
    );

    let journal: Arc<Mutex<ClientsEventMongoRepository>> = Arc::new(
        Mutex::new(
            ClientsEventMongoRepository {
                dao: ClientsEventMongoDAO::new("seedassure2035mongo".to_string(), "clients_journal_actix".to_string()).await
            }
        )
    );

    let clients_service: Arc<Mutex<ClientsServiceImpl<ClientsMongoRepository, ClientsEventMongoRepository>>> = Arc::new(
        Mutex::new(
            ClientsServiceImpl {
                store: Arc::clone(&store),
                journal: Arc::clone(&journal),
            }
        )
    );

    let engine_client: Arc<Mutex<Engine<ClientStates, ClientsCommands, ClientEvents, ClientsMongoRepository, ClientsEventMongoRepository>>> = Arc::new(Mutex::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateClientHandler {})),
            CommandHandler::Update(Box::new(UpdateClientHandler {})),
        ],
        reducer: ClientReducer::new().underlying,
        store: Arc::clone(&store),
        journal: Arc::clone(&journal)
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
        let jwt_token_service = JwtTokenService::new("test".to_string());


        App::new()
            .app_data(web::Data::new(Arc::clone(&engine_client)))
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(jwt_token_service))
            .app_data(
                web::Data::new(Arc::clone(&store))
            )
            .app_data(
                web::Data::new(Arc::clone(&journal))
            )
            .app_data(
                web::Data::new(Arc::clone(&clients_service))
            )
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .service(fetch_one)
            .service(fetch_many)
            .service(fetch_events)
            .service(insert_one)
            .service(update_one)
    })
        .workers(2)
        .bind((api_address, api_port))?
        .run()
        .await
}