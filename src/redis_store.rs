use crate::store::CaptchaStore;
use async_trait::async_trait;
use redis::{Client, RedisError};
use std::sync::Arc;

pub struct RedisStore {
  conn_mgr: Arc<Client>,
}

impl RedisStore {
  pub async fn new(redis_url: &str) -> Result<Self, RedisError> {
    let client = Client::open(redis_url)?;
    Ok(Self {
      conn_mgr: Arc::new(client),
    })
  }
}

#[async_trait]
impl CaptchaStore for RedisStore {
  async fn set(&self, key: String, value: String, ttl_secs: u64) {
    // let mut conn = self.conn_mgr.clone();
    // let mut conn_ref = Arc::get_mut(&mut conn).expect("Multiple references exist");
    // conn_ref.set_ex(&key, value, ttl_secs).await.expect("TODO: panic message");
  }

  async fn get(&self, key: &str) -> Option<String> {
    // let mut conn = self.conn_mgr.clone();
    // let mut conn_ref = Arc::get_mut(&mut conn).expect("Multiple references exist");
    // conn_ref.get(key)
    None
  }

  async fn remove(&self, key: &str) {
    // let mut conn = self.conn_mgr.clone();
    // let mut conn_ref = Arc::get_mut(&mut conn).expect("Multiple references exist");
    // let _: Result<(), _> = conn_ref.del(key).await;
  }
}
