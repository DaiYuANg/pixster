use crate::config::CaptchaConfig;
use moka::future::Cache;
use tokio::time::Duration;
use crate::store::Store;

pub struct InMemoryStore {
  cache: Cache<String, String>,
}

impl InMemoryStore {
  pub fn new(captcha_config: CaptchaConfig) -> Self {
    Self {
      // 设置最大容量，可按需调整，比如 10_000
      cache: Cache::builder()
        .time_to_live(Duration::from_secs(captcha_config.expire_seconds as u64))
        .weigher(|_k: &String, v: &String| v.len() as u32)
        .build(),
    }
  }
}

#[async_trait::async_trait]
impl Store for InMemoryStore {
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
