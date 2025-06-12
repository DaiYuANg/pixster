use crate::store::CaptchaStore;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

pub struct InMemoryStore {
  map: Arc<RwLock<HashMap<String, String>>>,
}

impl InMemoryStore {
  pub fn new() -> Self {
    Self {
      map: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}

#[async_trait::async_trait]
impl CaptchaStore for InMemoryStore {
  async fn set(&self, key: String, value: String, ttl_secs: u64) {
    let map = self.map.clone();
    map.write().await.insert(key.clone(), value);

    // 过期删除逻辑
    tokio::spawn(async move {
      sleep(Duration::from_secs(ttl_secs)).await;
      map.write().await.remove(&key);
    });
  }

  async fn get(&self, key: &str) -> Option<String> {
    self.map.read().await.get(key).cloned()
  }

  async fn remove(&self, key: &str) {
    self.map.write().await.remove(key);
  }
}
