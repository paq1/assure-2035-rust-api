use utoipa::Modify;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::api::clients::routes::read_routes::__path_fetch_events_client;
use crate::api::clients::routes::read_routes::__path_fetch_many_client;
use crate::api::clients::routes::read_routes::__path_fetch_one_client;
use crate::api::clients::routes::write_routes::__path_insert_one_client;
use crate::api::clients::routes::write_routes::__path_update_one_client;
use crate::api::contrats::routes::read_routes::__path_fetch_events_contrat;
use crate::api::contrats::routes::read_routes::__path_fetch_many_contrat;
use crate::api::contrats::routes::read_routes::__path_fetch_one_contrat;
use crate::api::contrats::routes::write_routes::__path_insert_one_contrat;
use crate::api::contrats::routes::write_routes::__path_update_one_contrat;
use crate::models::clients::views::*;
use crate::core::contrats::data::ContratStates;
use crate::models::clients::commands::*;
use crate::models::clients::views::ClientView;
use crate::models::contrats::commands::*;
use crate::models::contrats::views::ContratView;
use crate::models::shared::jsonapi::ManyView;
use crate::models::clients::shared::ClientData;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many_client,
        fetch_one_client,
        insert_one_client,
        update_one_client,
        fetch_events_client,
        fetch_many_contrat,
        fetch_one_contrat,
        insert_one_contrat,
        update_one_contrat,
        fetch_events_contrat
    ),
    components(
        schemas(
            ClientView,
            ManyView < ClientViewState >,
            CreateClientCommand,
            UpdateClientCommand,
            DeleteClientCommand,
            ContratView,
            ManyView < ContratStates >,
            CreateContratCommand,
            UpdateContratCommand,
            DeleteContratCommand,
            ClientData
        )
    ),
    modifiers(& SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            ),
        )
    }
}