use std::sync::Arc;
use async_trait::async_trait;
use crate::memory_store::InMemoryStore;
use crate::redis_store::RedisStore;

#[async_trait]
pub trait CaptchaStore: Send + Sync {
  async fn set(&self, key: String, value: String, ttl_secs: u64);
  async fn get(&self, key: &str) -> Option<String>;
  async fn remove(&self, key: &str);
}


pub enum StoreBackend {
  Memory,
  // Redis,
}

pub fn create_store(backend: StoreBackend) -> Arc<dyn CaptchaStore> {
  match backend {
    StoreBackend::Memory => Arc::new(InMemoryStore::new()),
    // StoreBackend::Redis => Arc::new(RedisStore::new(o)),
  }
}