use async_trait::async_trait;

#[async_trait]
pub trait ClientService: Send + Sync {}