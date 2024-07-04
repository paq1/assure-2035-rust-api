use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::clients::data::{ClientEvents, ClientStates};
use crate::core::clients::services::ClientService;
use crate::core::shared::context::Context;
use crate::core::shared::id_generator::IdGenerator;
use crate::core::shared::repositories::{ReadOnlyEntityRepo, WriteOnlyEntityRepo, WriteOnlyEventRepo};
use crate::models::clients::commands::*;
use crate::models::shared::errors::ResultErr;

pub struct ClientsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ClientStates, String> + ReadOnlyEntityRepo<ClientStates, String>,
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String>,
{
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>,
}

#[async_trait]
impl<STORE, JOURNAL> ClientService for ClientsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ClientStates, String> + ReadOnlyEntityRepo<ClientStates, String> + Send,
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String> + Send,
{
    async fn delete_client(&self, _command: DeleteClientCommand, _id: String, _ctx: Context) -> ResultErr<String> {
        todo!()
    }
}

impl<STORE, JOURNAL> IdGenerator for ClientsServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<ClientStates, String> + ReadOnlyEntityRepo<ClientStates, String>,
    JOURNAL: WriteOnlyEventRepo<ClientEvents, String>,
{
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
