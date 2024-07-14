use utoipa::Modify;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::api::clients::routes::read_routes::__path_fetch_events_client;
use crate::api::clients::routes::read_routes::__path_fetch_many_client;
use crate::api::clients::routes::read_routes::__path_fetch_one_client;
use crate::api::clients::routes::write_routes::__path_disable_one_client;
use crate::api::clients::routes::write_routes::__path_insert_one_client;
use crate::api::clients::routes::write_routes::__path_update_one_client;
use crate::api::contrats::routes::read_routes::__path_fetch_events_contrat;
use crate::api::contrats::routes::read_routes::__path_fetch_many_contrat;
use crate::api::contrats::routes::read_routes::__path_fetch_one_contract_event;
use crate::api::contrats::routes::read_routes::__path_fetch_one_contrat;
use crate::api::contrats::routes::write_routes::__path_approve_one_contrat;
use crate::api::contrats::routes::write_routes::__path_insert_one_contrat;
use crate::api::contrats::routes::write_routes::__path_update_one_contrat;
use crate::core::contrats::data::ContratStates;
use crate::core::shared::repositories::query::Paged;
use crate::models::clients::commands::*;
use crate::models::clients::shared::{ClientData, DisableReason, Phone, PhoneNumber};
use crate::models::clients::views::*;
use crate::models::clients::views::ClientView;
use crate::models::clients::views::ClientViewEvent;
use crate::models::contrats::commands::*;
use crate::models::contrats::shared::{ContractData, CurrencyValue, Vehicle};
use crate::models::contrats::views::ContratView;
use crate::models::shared::jsonapi::ManyView;
use crate::models::shared::views::command_handler_view::ApiView;
use crate::models::shared::views::DataWrapperView;
use crate::models::shared::views::entities::EntityView;
use crate::models::shared_business::Adresse;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many_client,
        fetch_one_client,
        insert_one_client,
        update_one_client,
        disable_one_client,
        fetch_events_client,
        fetch_many_contrat,
        fetch_one_contrat,
        insert_one_contrat,
        approve_one_contrat,
        update_one_contrat,
        fetch_events_contrat,
        fetch_one_contract_event,
    ),
    components(
        schemas(
            ClientView,
            ManyView < ClientViewState >,
            CreateClientCommand,
            UpdateClientCommand,
            DisableClientCommand,
            ContratView,
            ManyView < ContratStates >,
            CreateContratCommand,
            ApproveContractCommand,
            UpdateContratCommand,
            DeleteContratCommand,
            ClientData,
            Adresse,
            PhoneNumber,
            Phone,
            ContractData,
            Vehicle,
            CurrencyValue,
            DisableReason,
            DataWrapperView < ApiView < ClientViewEvent > >,
            EntityView<ClientViewState>,
            Paged<EntityView<ClientViewState>>,
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