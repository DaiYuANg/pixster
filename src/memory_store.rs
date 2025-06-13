use crate::store::CaptchaStore;
use moka::future::Cache;
use moka::policy::EvictionPolicy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, sleep};

pub struct InMemoryStore {
  cache: Cache<String, String>,
}

impl InMemoryStore {
  pub fn new() -> Self {
    Self {
      // 设置最大容量，可按需调整，比如 10_000
      cache: Cache::builder()
        .time_to_live(Duration::from_secs(120))
        .time_to_idle(Duration::from_secs(60))
        .weigher(|_k: &String, v: &String| v.len() as u32)
        .build(),
    }
  }
}

#[async_trait::async_trait]
impl CaptchaStore for InMemoryStore {
  async fn set(&self, key: String, value: String) {
    self.cache.insert(key, value).await;
  }

  async fn get(&self, key: &str) -> Option<String> {
    self.cache.get(key).await
  }

  async fn remove(&self, key: &str) {
    self.cache.invalidate(key).await;
  }
}
