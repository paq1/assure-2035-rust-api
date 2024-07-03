use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use futures::lock::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::shared::token::JwtTokenService;
use crate::api::swagger::ApiDoc;
use api::todos::routes::read_routes::{fetch_many, fetch_one};
use crate::api::todos::services::TodosServiceImpl;
use crate::api::todos::todo_event_mongo_repository::TodosEventMongoRepository;
use crate::api::todos::todos_mongo_dao::{TodosEventMongoDAO, TodosMongoDAO};
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use api::todos::routes::write_routes::{insert_one, update_one};
use crate::api::todos::routes::read_routes::fetch_events;
use crate::core::shared::event_sourcing::CommandHandler;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::core::todos::command_handler::command_handler_impl::{CreateTodoHandler, UpdateTodoHandler};
use crate::core::todos::data::{TodoEvents, TodoStates};
use crate::core::todos::reducer::TodoReducer;
use crate::models::shared::errors::StandardHttpError;
use crate::models::todos::commands::TodoCommands;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let store: Arc<Mutex<TodosMongoRepository>> = Arc::new(
        Mutex::new(
            TodosMongoRepository {
                dao: TodosMongoDAO::new("seedtodomongo".to_string(), "todos_store_actix".to_string()).await
            }
        )
    );

    let journal: Arc<Mutex<TodosEventMongoRepository>> = Arc::new(
        Mutex::new(
            TodosEventMongoRepository {
                dao: TodosEventMongoDAO::new("seedtodomongo".to_string(), "todos_journal_actix".to_string()).await
            }
        )
    );

    let todos_service: Arc<Mutex<TodosServiceImpl<TodosMongoRepository, TodosEventMongoRepository>>> = Arc::new(
        Mutex::new(
            TodosServiceImpl {
                store: Arc::clone(&store),
                journal: Arc::clone(&journal),
            }
        )
    );

    let engine_todo: Arc<Mutex<Engine<TodoStates, TodoCommands, TodoEvents, TodosMongoRepository, TodosEventMongoRepository>>> = Arc::new(Mutex::new(Engine {
        handlers: vec![
            CommandHandler::Create(Box::new(CreateTodoHandler {})),
            CommandHandler::Update(Box::new(UpdateTodoHandler {})),
        ],
        reducer: TodoReducer::new().underlying,
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
            .app_data(web::Data::new(Arc::clone(&engine_todo)))
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(jwt_token_service))
            .app_data(
                web::Data::new(Arc::clone(&store))
            )
            .app_data(
                web::Data::new(Arc::clone(&journal))
            )
            .app_data(
                web::Data::new(Arc::clone(&todos_service))
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