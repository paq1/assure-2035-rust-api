use utoipa::Modify;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::api::clients::routes::read_routes::__path_fetch_events_client;
use crate::api::clients::routes::read_routes::__path_fetch_many_client;
use crate::api::clients::routes::read_routes::__path_fetch_one_client;
use crate::api::clients::routes::write_routes::__path_insert_one_client;
use crate::api::clients::routes::write_routes::__path_update_one_client;
use crate::core::clients::data::ClientStates;
use crate::models::clients::commands::*;
use crate::models::clients::views::ClientView;
use crate::models::shared::jsonapi::Many;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many_client,
        fetch_one_client,
        insert_one_client,
        update_one_client,
        fetch_events_client
    ),
    components(
        schemas(
            ClientView,
            Many<ClientStates>,
            CreateClientCommand,
            UpdateClientCommand,
            DeleteClientCommand
        )
    ),
    modifiers(&SecurityAddon)
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
            )
        )
    }
}