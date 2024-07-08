use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use futures::lock::Mutex;
use uuid::Uuid;

use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use crate::api::shared::OwnUrl;
use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::core::clients::data::{ClientEvents, ClientStates};
use crate::core::shared::event_sourcing::engine::Engine;
use crate::models::clients::commands::{ClientsCommands, CreateClientCommand, UpdateClientCommand};
use crate::models::clients::views::ClientViewEvent;
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::views::command_handler_view::from_output_command_handler_to_view;

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
    jwt_token_service: web::Data<JwtRSATokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<ClientStates, ClientsCommands, ClientEvents, ClientsMongoRepository, ClientsEventMongoRepository>>>>,
    own_url: web::Data<OwnUrl>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let command = ClientsCommands::Create(body.into_inner());

            let entity_id = Uuid::new_v4().to_string();

            let event = engine.lock().await
                .compute(command, entity_id.clone(), "create-client".to_string(), ctx).await;

            match event {
                Ok((event, _state)) => HttpResponse::Created().json(from_output_command_handler_to_view::<ClientEvents, ClientViewEvent>(
                    own_url.url.clone(),
                    event,
                    "clients".to_string()
                )),
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
#[put("/clients/{entity_id}/commands/update")]
pub async fn update_one_client(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateClientCommand>,
    jwt_token_service: web::Data<JwtRSATokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<ClientStates, ClientsCommands, ClientEvents, ClientsMongoRepository, ClientsEventMongoRepository>>>>,
    own_url: web::Data<OwnUrl>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = ClientsCommands::Update(body.into_inner());

            let event = engine.lock().await
                .compute(command, id, "update-client".to_string(), ctx).await;

            match event {
                Ok((event, _state)) => HttpResponse::Ok().json(from_output_command_handler_to_view::<ClientEvents, ClientViewEvent>(own_url.url.clone(), event, "clients".to_string())),
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}
