use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Query;

use crate::api::clients::query::ClientQuery;
use crate::api::shared::helpers::context::CanDecoreFromHttpRequest;
use crate::api::shared::mappers::reponse_handler_view::from_output_command_handler_to_view;
use crate::api::shared::mappers::state_view::{CanBeManyView, from_states_to_entity_view, from_states_to_view};
use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::context::Context;
use crate::core::shared::repositories::entities::RepositoryEntity;
use crate::core::shared::repositories::events::RepositoryEvents;
use crate::core::shared::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use crate::core::shared::repositories::query::{Paged, Query as QueryCore};
use crate::models::clients::views::{ClientViewEvent, ClientViewState};
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::jsonapi::CanBeView;
use crate::models::shared::views::entities::EntityView;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Paged<EntityView<ClientViewState>>)
    ),
    params(
        ClientQuery
    )
)]
#[get("/clients")]
pub async fn fetch_many_client(
    store: web::Data<Arc<dyn RepositoryEntity<ClientStates, String>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ClientQuery>,
    req: HttpRequest,
) -> impl Responder {
    let ctx: Context = Context::empty()
        .decore_with_http_header(&req)
        .clone_with_filter(
            HashMap::from([
                ("page[number]".to_string(), query.number.map(|x| x.to_string()).unwrap_or("0".to_string())),
                ("page[size]".to_string(), query.size.map(|x| x.to_string()).unwrap_or("10".to_string())),
            ])
        );

    match store.fetch_many(
        query.into()
    ).await {
        Ok(items) => {
            let paged_view: Paged<EntityView<ClientViewState>> = items.map(|entity| {
                from_states_to_entity_view(entity, "clients".to_string(), &ctx)
            });

            HttpResponse::Ok().json(paged_view.to_many_view(&ctx, "clients".to_string(), HashMap::from([("clients".to_string(), "clients".to_string()), ("contracts".to_string(), "contracts".to_string())])))
        }
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

#[utoipa::path(
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = ClientStates
        )
    )
)]
#[get("/clients/{entity_id}")]
pub async fn fetch_one_client(
    path: web::Path<String>,
    store: web::Data<Arc<dyn RepositoryEntity<ClientStates, String>>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();

    let ctx = Context::empty().decore_with_http_header(&req);

    match store.fetch_one(id).await {
        Ok(Some(entity)) => {
            let view = from_states_to_view(entity, "clients".to_string(), &ctx);

            HttpResponse::Ok().json(view)
        }
        Ok(_) => HttpResponse::NotFound().json(http_error.not_found.clone()),
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

#[utoipa::path(
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = ClientView
        )
    ),
    params(
        ClientQuery
    )
)]
#[get("/clients/{entity_id}/events")]
pub async fn fetch_events_client(
    path: web::Path<String>,
    journal: web::Data<Arc<dyn RepositoryEvents<ClientEvents, String>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ClientQuery>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();
    let query_core: QueryCore = query.clone().into();

    let ctx: Context = Context::empty()
        .decore_with_http_header(&req)
        .clone_with_filter(
            HashMap::from([
                ("page[number]".to_string(), query.number.map(|x| x.to_string()).unwrap_or("0".to_string())),
                ("page[size]".to_string(), query.size.map(|x| x.to_string()).unwrap_or("10".to_string())),
            ])
        );

    let query_core_with_filter = QueryCore {
        filter: Filter::Expr(
            Expr::ExprStr(
                ExprGeneric::<String> {
                    field: "entity_id".to_string(),
                    operation: Operation::EqualsTo,
                    head: id.to_string(),
                }
            )
        ),
        ..query_core.clone()
    };

    match journal.fetch_many(query_core_with_filter).await {
        Ok(items) => {
            let paged_view = items.map(|x| {
                EntityView { // todo entity event view ici ? (a voir avec les specs s'il faut un différence entre la vu event / state
                    r#type: "org:example:insurance:client".to_string(), // fixme passer le client ontology
                    id: x.entity_id,
                    attributes: x.data.to_view(),
                    links: None,
                }
            });

            HttpResponse::Ok().json(paged_view.to_many_view(&ctx, "clients".to_string(), HashMap::new()))
        }
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}


#[utoipa::path(
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = DataWrapperView < EventView < ClientViewEvent >>
        )
    )
)]
#[get("/clients/{entity_id}/events/{event_id}")]
pub async fn fetch_one_client_event(
    path: web::Path<(String, String)>,
    journal: web::Data<Arc<dyn RepositoryEvents<ClientEvents, String>>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let (_, event_id) = path.into_inner();

    let ctx = Context::empty()
        .decore_with_http_header(&req);

    match journal.fetch_one(event_id).await {
        Ok(maybe_event) => {
            match maybe_event {
                Some(event) => {
                    let view = from_output_command_handler_to_view::<ClientEvents, ClientViewEvent>(
                        event,
                        "clients".to_string(),
                        "org:example:insurance:client".to_string(),
                        &ctx,
                    );
                    HttpResponse::Ok().json(view)
                }
                None => {
                    HttpResponse::InternalServerError().json(http_error.not_found.clone())
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}
