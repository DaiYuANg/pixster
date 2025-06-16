use crate::config::CaptchaConfig;
use crate::store::CaptchaStore;
use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client, RedisError};
use std::sync::Arc;

pub struct RedisStore {
  conn_mgr: Arc<ConnectionManager>,
  captcha_config: CaptchaConfig,
}

impl RedisStore {
  pub async fn new(redis_url: &str, captcha_config: CaptchaConfig) -> Result<Self, RedisError> {
    let client = Client::open(redis_url)?;
    let conn = client.get_connection_manager().await?;
    Ok(Self {
      conn_mgr: Arc::new(conn),
      captcha_config,
    })
  }
}

#[async_trait]
impl CaptchaStore for RedisStore {
  async fn set(&self, key: String, value: String) {
    let mut conn = (*self.conn_mgr).clone();
    let _: () = conn
      .set_ex(key, value, self.captcha_config.expire_seconds as u64)
      .await
      .unwrap();
  }

  async fn get(&self, key: &str) -> Option<String> {
    let mut conn = (*self.conn_mgr).clone();
    conn.get(key).await.ok()
  }

  async fn remove(&self, key: &str) {
    let mut conn = (*self.conn_mgr).clone();
    let _: () = conn.del(key).await.unwrap_or_default();
  }
}
