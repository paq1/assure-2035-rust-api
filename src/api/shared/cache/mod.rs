use moka::future::Cache;

#[derive(Clone)]
pub struct CacheAsync {
    pub underlying: Cache<String, String>,
}

impl CacheAsync {
    pub async fn get(&self, key: &String) -> Option<String> {
        self.underlying.get(key).await
    }

    pub async fn upsert(&self, key: String, id: String) {
        self.underlying.insert(key, id).await;
    }
}
