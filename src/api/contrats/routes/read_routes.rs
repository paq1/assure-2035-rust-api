use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Query;
use futures::lock::Mutex;

use crate::api::contrats::query::ContratQuery;
use crate::api::shared::helpers::context::CanDecoreFromHttpRequest;
use crate::api::shared::mappers::state_view::{CanBeManyView, from_states_to_entity_view, from_states_to_view};
use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::shared::context::Context;
use crate::core::shared::repositories::entities::RepositoryEntity;
use crate::core::shared::repositories::events::RepositoryEvents;
use crate::core::shared::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use crate::core::shared::repositories::query::Query as QueryCore;
use crate::models::contrats::views::ContractViewEvent;
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::jsonapi::CanBeView;
use crate::models::shared::views::command_handler_view::from_output_command_handler_to_view;
use crate::models::shared::views::entities::EntityView;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < ContratStates >)
    ),
    params(
        ContratQuery
    )
)]
#[get("/contracts")]
pub async fn fetch_many_contrat(
    store: web::Data<Arc<Mutex<dyn RepositoryEntity<ContratStates, String>>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ContratQuery>,
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

    let store_lock = store.lock().await;
    match store_lock.fetch_many(
        query.into()
    ).await {
        Ok(items) => {
            let paged_view = items.map(|entity| {
                from_states_to_entity_view(entity, "contracts".to_string(), &ctx)
            });

            HttpResponse::Ok().json(paged_view.to_many_view(&ctx, "contracts".to_string(), HashMap::from([("clients".to_string(), "clients".to_string()), ("contracts".to_string(), "contracts".to_string())])))
        },
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

#[utoipa::path(
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = ContratStates
        )
    )
)]
#[get("/contracts/{entity_id}")]
pub async fn fetch_one_contrat(
    path: web::Path<String>,
    repo: web::Data<Arc<Mutex<dyn RepositoryEntity<ContratStates, String>>>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest
) -> impl Responder {
    let id = path.into_inner();

    let repo_lock = repo.lock().await;


    let ctx = Context::empty().decore_with_http_header(&req);


    match repo_lock.fetch_one(id).await {
        Ok(Some(entity)) => {
            let view = from_states_to_view(entity, "contracts".to_string(), &ctx);

            HttpResponse::Ok().json(view)
        },
        Ok(_) => HttpResponse::NotFound().json(http_error.not_found.clone()),
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

#[utoipa::path(
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = ContratView
        )
    ),
    params(
        ContratQuery
    )
)]
#[get("/contracts/{entity_id}/events")]
pub async fn fetch_events_contrat(
    path: web::Path<String>,
    journal: web::Data<Arc<Mutex<dyn RepositoryEvents<ContratEvents, String>>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ContratQuery>,
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


    let id = path.into_inner();
    let query_core: QueryCore = query.into();

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


    let journal_lock = journal.lock().await;
    match journal_lock.fetch_many(query_core_with_filter).await {
        Ok(items) => {
            let paged_view = items.map(|x| {
                EntityView { // todo entity event view ici ? (a voir avec les specs s'il faut un différence entre la vu event / state
                    r#type: "org:example:insurance:contract".to_string(), // fixme passer le client ontology
                    id: x.entity_id,
                    attributes: x.data.to_view(),
                    links: None
                }
            });

            HttpResponse::Ok().json(paged_view.to_many_view(&ctx, "clients".to_string(), HashMap::new()))
        },
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

#[utoipa::path(
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = DataWrapperView < EventView < ContractViewEvent >>
        )
    )
)]
#[get("/contracts/{entity_id}/events/{event_id}")]
pub async fn fetch_one_contract_event(
    path: web::Path<(String, String)>,
    journal: web::Data<Arc<Mutex<dyn RepositoryEvents<ContratEvents, String>>>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let (_, event_id) = path.into_inner();
    let journal_lock = journal.lock().await;

    let ctx = Context::empty()
        .decore_with_http_header(&req);

    match journal_lock.fetch_one(event_id).await {
        Ok(maybe_event) => {
            match maybe_event {
                Some(event) => {
                    let view = from_output_command_handler_to_view::<ContratEvents, ContractViewEvent>(
                        event,
                        "contracts".to_string(),
                        "org:example:insurance:contract".to_string(),
                        &ctx
                    );
                    HttpResponse::Ok().json(view)
                },
                None => {
                    HttpResponse::InternalServerError().json(http_error.not_found.clone())
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

