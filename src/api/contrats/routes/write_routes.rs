use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use futures::lock::Mutex;
use uuid::Uuid;

use crate::api::contrats::contrats_event_mongo_repository::ContratsEventMongoRepository;
use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::shared::event_sourcing::engine::Engine;
use crate::models::contrats::commands::{ContratsCommands, CreateContratCommand, UpdateContratCommand};
use crate::models::contrats::views::ContractViewEvent;
use crate::models::shared::errors::{Error, StandardHttpError};
use crate::models::shared::views::command_handler_view::from_output_command_handler_to_view;

#[utoipa::path(
    request_body = CreateContratCommand,
    responses(
    (status = 201, description = "fait ca", body = ContratView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/contracts/commands/create")]
pub async fn insert_one_contrat(
    req: HttpRequest,
    body: web::Json<CreateContratCommand>,
    jwt_token_service: web::Data<JwtRSATokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<ContratStates, ContratsCommands, ContratEvents, ContratsEventMongoRepository>>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let command = ContratsCommands::Create(body.into_inner());

            let entity_id = Uuid::new_v4().to_string();

            let event = engine.lock().await
                .compute(command, entity_id.clone(), "create-contrat".to_string(), &ctx).await;

            match event {
                Ok((event, _state)) => {
                    HttpResponse::Created().json(
                        // fixme faire un view pour le contract
                        from_output_command_handler_to_view::<ContratEvents, ContractViewEvent>(
                            event,
                            "contracts".to_string(),
                            "org:example:insurance:contract".to_string(),
                            &ctx,
                        )
                    )
                }
                Err(err) => {

                    match err {
                        Error::Http(e) => {
                            match e.status {
                                Some(404) => HttpResponse::NotFound().json(e),
                                _ => HttpResponse::InternalServerError().json(e)
                            }
                        },
                        _ => {
                            println!("creation impossible !");
                            println!("reasons : {err:?}");
                            HttpResponse::InternalServerError().json(err)
                        }
                    }
                }
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

#[utoipa::path(
    request_body = UpdateContratCommand,
    responses(
    (status = 200, description = "fait ca", body = ContratView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/contracts/commands/update/{entity_id}")]
pub async fn update_one_contrat(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateContratCommand>,
    jwt_token_service: web::Data<JwtRSATokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<ContratStates, ContratsCommands, ContratEvents, ContratsEventMongoRepository>>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = ContratsCommands::Update(body.into_inner());

            let event = engine.lock().await
                .compute(command, id, "update-contrat".to_string(), &ctx).await;

            match event {
                Ok((event, _state)) => {
                    HttpResponse::Created().json(
                        // fixme faire un view pour le contract
                        from_output_command_handler_to_view::<ContratEvents, ContractViewEvent>(
                            event,
                            "contracts".to_string(),
                            "org:example:insurance:contract".to_string(),
                            &ctx,
                        )
                    )
                }
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

