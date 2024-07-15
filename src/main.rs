use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use moka::future::Cache;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::clients::routes::read_routes::{fetch_many_client, fetch_one_client};
use api::clients::routes::write_routes::{insert_one_client, update_one_client};

use crate::api::clients::client_component::ClientComponent;
use crate::api::clients::routes::read_routes::{fetch_events_client, fetch_one_client_event};
use crate::api::clients::routes::write_routes::disable_one_client;
use crate::api::contrats::contract_component::ContractComponent;
use crate::api::contrats::routes::read_routes::{fetch_events_contrat, fetch_many_contrat, fetch_one_contract_event, fetch_one_contrat};
use crate::api::contrats::routes::write_routes::{approve_one_contrat, insert_one_contrat, reject_one_contrat, terminate_one_contrat, update_one_contrat};
use crate::api::shared::cache::CacheAsync;
use crate::api::shared::token::services::jwt_hmac::JwtHMACTokenService;
use crate::api::shared::token::services::jwt_rsa::JwtRSATokenService;
use crate::api::swagger::ApiDoc;
use crate::models::shared::errors::StandardHttpError;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let auth_back_url = std::env::var("AUTH_BACK_URL").unwrap_or("http://localhost:9001".to_string());

    let cache = Arc::new(CacheAsync { underlying: Cache::new(10_000) });
    let http_client = Arc::new(reqwest::Client::new());

    // client ontology
    // dao
    let client_component = ClientComponent::new().await;
    let contract_component = ContractComponent::new(Arc::clone(&client_component.store)).await;

    let openapi = ApiDoc::openapi();
    let api_address = std::env::var("API_ADDRESS").unwrap();
    let api_port = std::env::var("API_PORT").unwrap().parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        let standard_http_error = StandardHttpError::new();
        let jwt_token_service = JwtHMACTokenService::new("test".to_string());
        let jwt_rsa_token_service = JwtRSATokenService::new(Arc::clone(&cache), Arc::clone(&http_client), auth_back_url.clone());


        App::new()
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .app_data(web::Data::new(jwt_rsa_token_service))
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(jwt_token_service))
            // clients services
            .app_data(web::Data::new(Arc::clone(&client_component.engine)))
            .app_data(
                web::Data::new(Arc::clone(&client_component.store))
            )
            .app_data(
                web::Data::new(Arc::clone(&client_component.journal))
            )
            .app_data(
                web::Data::new(Arc::clone(&client_component.service))
            )
            // contrats services
            .app_data(web::Data::new(Arc::clone(&contract_component.engine)))
            .app_data(
                web::Data::new(Arc::clone(&contract_component.store))
            )
            .app_data(
                web::Data::new(Arc::clone(&contract_component.journal))
            )
            .app_data(
                web::Data::new(Arc::clone(&contract_component.service))
            )
            // client routes
            .service(fetch_one_client)
            .service(fetch_one_client_event)
            .service(fetch_many_client)
            .service(fetch_events_client)
            .service(insert_one_client)
            .service(update_one_client)
            .service(disable_one_client)
            // contrats routes
            .service(fetch_one_contract_event)
            .service(fetch_one_contrat)
            .service(fetch_many_contrat)
            .service(fetch_events_contrat)
            .service(insert_one_contrat)
            .service(approve_one_contrat)
            .service(reject_one_contrat)
            .service(terminate_one_contrat)
            .service(update_one_contrat)
    })
        .workers(2)
        .bind((api_address, api_port))?
        .run()
        .await
}