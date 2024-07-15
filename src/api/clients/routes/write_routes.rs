use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use uuid::Uuid;

use crate::api::shared::mappers::reponse_handler_view::from_output_command_handler_to_view;
use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::models::clients::commands::{ClientsCommands, CreateClientCommand, DisableClientCommand, UpdateClientCommand};
use crate::models::clients::views::ClientViewEvent;
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
    jwt_token_service: web::Data<JwtRSATokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Engine<ClientStates, ClientsCommands, ClientEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let command = ClientsCommands::Create(body.into_inner());

            let entity_id = Uuid::new_v4().to_string();

            let event = engine
                .compute(command, entity_id.clone(), "create-client".to_string(), &ctx).await;

            match event {
                Ok((event, _state)) =>
                    HttpResponse::Created().json(
                        from_output_command_handler_to_view::<ClientEvents, ClientViewEvent>(
                            event,
                            "clients".to_string(),
                            "org:example:insurance:client".to_string(),
                            &ctx,
                        )
                    ),
                Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error)
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
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
    engine: web::Data<Arc<Engine<ClientStates, ClientsCommands, ClientEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = ClientsCommands::Update(body.into_inner());

            let event = engine
                .compute(command, id, "update-client".to_string(), &ctx).await;

            match event {
                Ok((event, _state)) => HttpResponse::Ok()
                    .json(
                        from_output_command_handler_to_view::<ClientEvents, ClientViewEvent>(
                            event,
                            "clients".to_string(),
                            "org:example:insurance:client".to_string(),
                            &ctx,
                        )
                    ),
                Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error)
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
    }
}

#[utoipa::path(
    request_body = DisableClientCommand,
    responses(
    (status = 200, description = "fait ca", body = ClientView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/clients/{entity_id}/commands/disable")]
pub async fn disable_one_client(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<DisableClientCommand>,
    jwt_token_service: web::Data<JwtRSATokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Engine<ClientStates, ClientsCommands, ClientEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = ClientsCommands::Disable(body.into_inner());

            let event = engine
                .compute(command, id, "disable-client".to_string(), &ctx).await;

            match event {
                Ok((event, _state)) => HttpResponse::Ok().json(
                    from_output_command_handler_to_view::<ClientEvents, ClientViewEvent>(
                        event,
                        "clients".to_string(),
                        "org:example:insurance:client".to_string(),
                        &ctx,
                    )),
                Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error)
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
    }
}

