use async_trait::async_trait;

use crate::core::clients::services::ClientService;

pub struct ClientsServiceImpl {}

#[async_trait]
impl ClientService for ClientsServiceImpl {}
