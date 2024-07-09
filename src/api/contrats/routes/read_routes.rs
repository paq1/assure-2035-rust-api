use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::Query;
use futures::lock::Mutex;
use crate::api::contrats::contrats_event_mongo_repository::ContratsEventMongoRepository;
use crate::api::contrats::contrats_mongo_repository::ContratsMongoRepository;
use crate::api::contrats::query::ContratQuery;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEntityRepo};
use crate::core::shared::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use crate::core::shared::repositories::query::Query as QueryCore;
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::jsonapi::ManyView;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < ContratStates >)
    ),
    params(
        ContratQuery
    )
)]
#[get("/contrats")]
pub async fn fetch_many_contrat(
    store: web::Data<Arc<Mutex<ContratsMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ContratQuery>,
) -> impl Responder {

    let store_lock = store.lock().await;
    match store_lock.fetch_many(query.into(), HashMap::new()).await {
        Ok(items) => HttpResponse::Ok().json(ManyView::new(items)),
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
#[get("/contrats/{entity_id}")]
pub async fn fetch_one_contrat(path: web::Path<String>, repo: web::Data<Arc<Mutex<ContratsMongoRepository>>>, http_error: web::Data<StandardHttpError>) -> impl Responder {
    let id = path.into_inner();

    let repo_lock = repo.lock().await;


    match repo_lock.fetch_one(id).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res.clone()),
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
#[get("/contrats/{entity_id}/events")]
pub async fn fetch_events_contrat(
    path: web::Path<String>,
    journal: web::Data<Arc<Mutex<ContratsEventMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<ContratQuery>,
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
    match journal_lock.fetch_many(query_core_with_filter, HashMap::new()).await {
        Ok(items) => HttpResponse::Ok().json(ManyView::new(items)),
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}
