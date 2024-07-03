use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use futures::lock::Mutex;
use uuid::Uuid;
use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::JwtTokenService;
use crate::api::todos::todo_event_mongo_repository::TodosEventMongoRepository;
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::core::todos::data::{TodoEvents, TodoStates};
use crate::models::shared::errors::StandardHttpError;
use crate::models::todos::commands::{CreateTodoCommand, TodoCommands, UpdateTodoCommand};
use crate::models::todos::views::Todo;

#[utoipa::path(
    request_body = CreateTodoCommand,
    responses(
    (status = 201, description = "fait ca", body = Todo),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/todos/commands/create")]
pub async fn insert_one(
    req: HttpRequest,
    body: web::Json<CreateTodoCommand>,
    jwt_token_service: web::Data<JwtTokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<TodoStates, TodoCommands, TodoEvents, TodosMongoRepository, TodosEventMongoRepository>>>>
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()) {
        Ok(ctx) => {
            let command = TodoCommands::Create(body.into_inner());

            let entity_id = Uuid::new_v4().to_string();

            let event = engine.lock().await
                .compute(command, entity_id.clone(), "create".to_string(), ctx).await;

            match event {
                Ok(_res) => HttpResponse::Created().json(Todo { name: entity_id }),
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

#[utoipa::path(
    request_body = UpdateTodoCommand,
    responses(
    (status = 200, description = "fait ca", body = Todo),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/todos/commands/update/{entity_id}")]
pub async fn update_one(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateTodoCommand>,
    jwt_token_service: web::Data<JwtTokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<TodoStates, TodoCommands, TodoEvents, TodosMongoRepository, TodosEventMongoRepository>>>>
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()) {
        Ok(ctx) => {

            let id = path.into_inner();
            let command = TodoCommands::Update(body.into_inner());

            let event = engine.lock().await
                .compute(command, id, "update".to_string(), ctx).await;

            match event {
                Ok(_res) => HttpResponse::Ok().json(Todo { name: "xxx".to_string() }),
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

