use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use futures::lock::Mutex;
use uuid::Uuid;

use crate::api::shared::helpers::http_response::{CanToHttpResponse, HttpKindResponse};
use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::core::contrats::command_handler::approve_command_handler::ApproveContractHandler;
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::shared::event_sourcing::engine::Engine;
use crate::models::contrats::commands::{ApproveContractCommand, ContratsCommands, CreateContratCommand, UpdateContratCommand};
use crate::models::contrats::views::ContractViewEvent;
use crate::models::shared::errors::StandardHttpError;
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
    engine: web::Data<Arc<Mutex<Engine<ContratStates, ContratsCommands, ContratEvents>>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let command = ContratsCommands::Create(body.into_inner());

            let entity_id = Uuid::new_v4().to_string();

            let event = engine.lock().await
                .compute(command, entity_id.clone(), "create-contrat".to_string(), &ctx).await;

            event.map(|(event, _)| {
                from_output_command_handler_to_view::<ContratEvents, ContractViewEvent>(
                    event,
                    "contracts".to_string(),
                    "org:example:insurance:contract".to_string(),
                    &ctx,
                )
            })
                .to_http_response_with_error_mapping(HttpKindResponse::Created)
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

#[utoipa::path(
    request_body = ApproveContractCommand,
    responses(
    (status = 200, description = "fait ca", body = ContratView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/contracts/{entity_id}/commands/approve")]
pub async fn approve_one_contrat(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<ApproveContractCommand>,
    jwt_token_service: web::Data<JwtRSATokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Mutex<Engine<ContratStates, ContratsCommands, ContratEvents>>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let command = ContratsCommands::Approve(body.into_inner());

            let entity_id = path.into_inner();

            let event = engine.lock().await
                .compute(command, entity_id.clone(), ApproveContractHandler::get_name().to_string(), &ctx).await;

            event.map(|(event, _)| {
                from_output_command_handler_to_view::<ContratEvents, ContractViewEvent>(
                    event,
                    "contracts".to_string(),
                    "org:example:insurance:contract".to_string(),
                    &ctx,
                )
            })
                .to_http_response_with_error_mapping(HttpKindResponse::Ok)
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
    engine: web::Data<Arc<Mutex<Engine<ContratStates, ContratsCommands, ContratEvents>>>>,
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

