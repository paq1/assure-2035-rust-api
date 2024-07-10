use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Query;
use futures::lock::Mutex;

use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use crate::api::clients::query::ClientQuery;
use crate::api::shared::helpers::context::CanDecoreFromHttpRequest;
use crate::core::clients::data::ClientEvents;
use crate::core::shared::context::Context;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEntityRepo, ReadOnlyEventRepo};
use crate::core::shared::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use crate::core::shared::repositories::query::Query as QueryCore;
use crate::models::clients::views::ClientViewEvent;
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::jsonapi::{CanBeView, ManyView};
use crate::models::shared::views::command_handler_view::from_output_command_handler_to_view;
use crate::models::shared::views::entities::EntityView;
use crate::models::shared::views::get_view::from_states_to_view;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < ClientStates >)
    ),
    params(
        ClientQuery
    )
)]
#[get("/clients")]
pub async fn fetch_many_client(
    store: web::Data<Arc<Mutex<ClientsMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ClientQuery>,
    req: HttpRequest,
) -> impl Responder {

    let ctx: Context = Context::empty().decore_with_http_header(&req);

    let store_lock = store.lock().await;
    match store_lock.fetch_many(
        query.into(),
        HashMap::from([("clients".to_string(), "clients".to_string()), ("contracts".to_string(), "contracts".to_string())]
        )
    ).await {
        Ok(items) => {
            let paged_view = items.map(|x| {
                EntityView {
                    r#type: "org:example:insurance:client".to_string(),
                    id: x.entity_id,
                    attributes: x.data.to_view(),
                    links: None
                }
            });

            HttpResponse::Ok().json(ManyView::new(paged_view, &ctx, "clients".to_string()))
        },
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
    repo: web::Data<Arc<Mutex<ClientsMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();

    let repo_lock = repo.lock().await;

    let ctx = Context::empty().decore_with_http_header(&req);


    match repo_lock.fetch_one(id).await {
        Ok(Some(entity)) => {
            let view = from_states_to_view(entity, "clients".to_string(), &ctx);

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
    journal: web::Data<Arc<Mutex<ClientsEventMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ClientQuery>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();
    let query_core: QueryCore = query.into();

    let ctx: Context = Context::empty().decore_with_http_header(&req);

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
    match journal_lock.fetch_many(query_core_with_filter, HashMap::new()).await {



        Ok(items) => {

            let paged_view = items.map(|x| {
                EntityView { // todo entity event view ici ? (a voir avec les specs s'il faut un différence entre la vu event / state
                    r#type: "org:example:insurance:client".to_string(), // fixme passer le client ontology
                    id: x.entity_id,
                    attributes: x.data.to_view(),
                    links: None
                }
            });

            HttpResponse::Ok().json(ManyView::new(paged_view, &ctx, "clients".to_string()))
        },
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
    journal: web::Data<Arc<Mutex<ClientsEventMongoRepository>>>,
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
                    let view = from_output_command_handler_to_view::<ClientEvents, ClientViewEvent>(
                        event,
                        "clients".to_string(),
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
