use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use futures::lock::Mutex;
use uuid::Uuid;

use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::services::jwt_hmac::JwtHMACTokenService;
use crate::core::clients::data::{ClientEvents, ClientStates};
use crate::core::shared::event_sourcing::engine::Engine;
use crate::models::clients::commands::{ClientsCommands, CreateClientCommand, UpdateClientCommand};
use crate::models::clients::views::ClientView;
use crate::models::shared::errors::StandardHttpError;

#[utoipa::path(
    request_body = CreateClientCommand,
    responses(
    (status = 201, description = "fait ca", body = ClientView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/clients/commands/create")]
pub async fn insert_one_client(
    req: HttpRequest,
    body: web::Json<CreateClientCommand>,
    jwt_token_service: web::Data<JwtHMACTokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<ClientStates, ClientsCommands, ClientEvents, ClientsMongoRepository, ClientsEventMongoRepository>>>>
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let command = ClientsCommands::Create(body.into_inner());

            let entity_id = Uuid::new_v4().to_string();

            let event = engine.lock().await
                .compute(command, entity_id.clone(), "create-client".to_string(), ctx).await;

            match event {
                Ok(_res) => HttpResponse::Created().json(ClientView { name: entity_id }),
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

#[utoipa::path(
    request_body = UpdateClientCommand,
    responses(
    (status = 200, description = "fait ca", body = ClientView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/clients/commands/update/{entity_id}")]
pub async fn update_one_client(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateClientCommand>,
    jwt_token_service: web::Data<JwtHMACTokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<ClientStates, ClientsCommands, ClientEvents, ClientsMongoRepository, ClientsEventMongoRepository>>>>
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {

            let id = path.into_inner();
            let command = ClientsCommands::Update(body.into_inner());

            let event = engine.lock().await
                .compute(command, id, "update-client".to_string(), ctx).await;

            match event {
                Ok(_res) => HttpResponse::Ok().json(ClientView { name: "xxx".to_string() }),
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

