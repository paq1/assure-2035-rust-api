use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::Query;
use futures::lock::Mutex;
use futures::stream::iter;
use crate::api::clients::clients_event_mongo_repository::ClientsEventMongoRepository;
use crate::api::clients::clients_mongo_repository::ClientsMongoRepository;
use crate::api::clients::query::ClientQuery;
use crate::api::shared::OwnUrl;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEntityRepo};
use crate::core::shared::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use crate::core::shared::repositories::query::Query as QueryCore;
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::jsonapi::{CanBeView, ManyView};
use crate::models::shared::views::entities::{EntityView, LinksEntityView};
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
) -> impl Responder {

    let store_lock = store.lock().await;
    match store_lock.fetch_many(query.into()).await {
        Ok(items) => {
            let paged_view = items.map(|x| {
                EntityView {
                    r#type: "org:example:insurance:client".to_string(),
                    id: x.entity_id,
                    attributes: x.data.to_view(),
                    links: None
                }
            });

            HttpResponse::Ok().json(ManyView::new(paged_view))
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
    own_url: web::Data<OwnUrl>,
) -> impl Responder {
    let id = path.into_inner();

    let repo_lock = repo.lock().await;


    match repo_lock.fetch_one(id).await {
        Ok(Some(entity)) => {
            let view = from_states_to_view(own_url.url.clone(), entity, "clients".to_string());

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
) -> impl Responder {
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
                    r#type: "org:example:insurance:client".to_string(),
                    id: x.entity_id,
                    attributes: x.data.to_view(),
                    links: None
                }
            });

            HttpResponse::Ok().json(ManyView::new(paged_view))
        },
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}
