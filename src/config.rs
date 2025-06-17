use dotenvy::dotenv;
use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use serde::Deserialize;
use serde_derive::Serialize;
use tracing::debug;
use crate::store::StoreBackend;

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(default)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub captcha: CaptchaConfig,
  pub store: StoreConfig,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CaptchaConfig {
  pub expire_seconds: usize,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ServerConfig {
  pub host: String,
  pub port: u16,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct StoreConfig {
  pub backend: StoreBackend,
  pub url: String,
}

impl Default for AppConfig {
  fn default() -> Self {
    AppConfig {
      server: ServerConfig {
        host: "127.0.0.1".into(),
        port: 3000,
      },
      captcha: CaptchaConfig {
        expire_seconds: 120,
      },
      store: StoreConfig {
        backend: StoreBackend::Memory,
        url: "redis://localhost:6379".into(),
      },
    }
  }
}

pub fn load_config() -> AppConfig {
  let env_prefix = "CAPSTER_";
  dotenv().ok();
  std::env::vars()
    .filter(|(k, _)| k.starts_with(env_prefix))
    .for_each(|(k, v)| debug!("{} = {}", k, v));
  let figment = Figment::from(Serialized::defaults(AppConfig::default()))
    .merge(Toml::file("capster.toml").nested())
    .merge(Env::prefixed(env_prefix).split("_"));
  let config = figment.extract().expect("配置加载失败");
  config
}
