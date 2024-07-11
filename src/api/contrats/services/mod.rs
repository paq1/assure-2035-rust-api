use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::contrats::services::ContratService;
use crate::core::contrats::services::formule_service::FormuleService;
use crate::core::shared::context::Context;
use crate::core::shared::id_generator::IdGenerator;
use crate::core::shared::repositories::{ReadOnlyEntityRepo, WriteOnlyEntityRepo, WriteOnlyEventRepo};
use crate::models::contrats::commands::DeleteContratCommand;
use crate::models::shared::errors::ResultErr;

pub mod formule_service_impl;
pub mod formule_repo_mock;

pub struct ContratsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ContratStates, String> + ReadOnlyEntityRepo<ContratStates, String>,
    JOURNAL: WriteOnlyEventRepo<ContratEvents, String>,
{
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>,
    pub formule_service: Arc<Mutex<Box<dyn FormuleService>>>,
}

#[async_trait]
impl<STORE, JOURNAL> ContratService for ContratsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ContratStates, String> + ReadOnlyEntityRepo<ContratStates, String> + Send,
    JOURNAL: WriteOnlyEventRepo<ContratEvents, String> + Send,
{
    async fn delete_contrat(&self, _command: DeleteContratCommand, _id: String, _ctx: Context) -> ResultErr<String> {
        todo!()
    }
}

impl<STORE, JOURNAL> IdGenerator for ContratsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ContratStates, String> + ReadOnlyEntityRepo<ContratStates, String>,
    JOURNAL: WriteOnlyEventRepo<ContratEvents, String>
{
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
