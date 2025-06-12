use crate::store::CaptchaStore;
use async_trait::async_trait;
use redb::{Database, TableDefinition};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("captcha");

pub struct RedbStore {
  db: Database,
}

impl RedbStore {
  pub fn new(db: Database) -> RedbStore {
    RedbStore { db }
  }
}

#[async_trait]
impl CaptchaStore for RedbStore {
  async fn set(&self, key: String, value: String, ttl_secs: u64) {
  }

  async fn get(&self, key: &str) -> Option<String> {
    None
  }

  async fn remove(&self, key: &str) {
  }
}
