use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::Query;
use futures::lock::Mutex;

use crate::api::todos::query::TodoQuery;
use crate::api::todos::todo_event_mongo_repository::TodosEventMongoRepository;
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::shared::repositories::{CanFetchMany, ReadOnlyEntityRepo};
use crate::core::shared::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use crate::core::shared::repositories::query::Query as QueryCore;
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::jsonapi::Many;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < TodoStates >)
    ),
    params(
        TodoQuery
    )
)]
#[get("/todos")]
pub async fn fetch_many(
    store: web::Data<Arc<Mutex<TodosMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<TodoQuery>,
) -> impl Responder {

    let store_lock = store.lock().await;
    match store_lock.fetch_many(query.into()).await {
        Ok(items) => HttpResponse::Ok().json(Many::new(items)),
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

#[utoipa::path(
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = Todo
        )
    )
)]
#[get("/todos/{entity_id}")]
pub async fn fetch_one(path: web::Path<String>, repo: web::Data<Arc<Mutex<TodosMongoRepository>>>, http_error: web::Data<StandardHttpError>) -> impl Responder {
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
        body = Todo
        )
    ),
    params(
        TodoQuery
    )
)]
#[get("/todos/{entity_id}/events")]
pub async fn fetch_events(
    path: web::Path<String>,
    journal: web::Data<Arc<Mutex<TodosEventMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<TodoQuery>,
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
        Ok(items) => HttpResponse::Ok().json(Many::new(items)),
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}
