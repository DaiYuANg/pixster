mod memory_store;
mod redis_store;

use crate::config::{CaptchaConfig, StoreConfig};
use crate::store::memory_store::InMemoryStore;
use crate::store::redis_store::RedisStore;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait Store: Send + Sync {
  async fn set(&self, key: String, value: String);
  async fn get(&self, key: &str) -> Option<String>;
  async fn remove(&self, key: &str);
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum StoreBackend {
  Memory,
  Redis,
}

pub async fn create_store(config: StoreConfig, captcha_config: CaptchaConfig) -> Arc<dyn Store> {
  let backend = config.backend;
  match backend {
    StoreBackend::Memory => Arc::new(InMemoryStore::new(captcha_config)),
    StoreBackend::Redis => {
      let redis_store = RedisStore::new(config.url.as_ref(), captcha_config)
        .await
        .unwrap();
      Arc::new(redis_store)
    }
  }
}
