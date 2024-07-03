use utoipa::Modify;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::api::todos::routes::read_routes::__path_fetch_many;
use crate::api::todos::routes::read_routes::__path_fetch_events;
use crate::api::todos::routes::read_routes::__path_fetch_one;
use crate::api::todos::routes::write_routes::__path_insert_one;
use crate::api::todos::routes::write_routes::__path_update_one;
use crate::models::todos::commands::{CreateTodoCommand, UpdateTodoCommand, DeleteTodoCommand};
use crate::models::todos::views::{Todo, TokenClaims};
use crate::models::shared::jsonapi::Many;
use crate::core::todos::data::TodoStates;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many,
        fetch_one,
        insert_one,
        update_one,
        fetch_events
    ),
    components(
        schemas(
            TokenClaims,
            Todo,
            Many<TodoStates>,
            Todo,
            CreateTodoCommand,
            UpdateTodoCommand,
            DeleteTodoCommand
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