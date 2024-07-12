use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::clients::data::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::clients::services::ClientService;
use crate::core::shared::context::Context;
use crate::core::shared::id_generator::IdGenerator;
use crate::core::shared::repositories::{RepositoryEntity, WriteOnlyEventRepo};
use crate::models::clients::commands::*;
use crate::models::shared::errors::ResultErr;

pub struct ClientsServiceImpl<JOURNAL>
where
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String>,
{
    pub store: Arc<Mutex<dyn RepositoryEntity<ClientStates, String>>>,
    pub journal: Arc<Mutex<JOURNAL>>,
}

#[async_trait]
impl<JOURNAL> ClientService for ClientsServiceImpl<JOURNAL>
where
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String> + Send,
{
    async fn delete_client(&self, _command: DisableClientCommand, _id: String, _ctx: Context) -> ResultErr<String> {
        todo!()
    }
}

impl<JOURNAL> IdGenerator for ClientsServiceImpl<JOURNAL>
where
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String>,
{
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
