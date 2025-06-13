use crate::store::StoreBackend;
use dotenvy::dotenv;
use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use serde::Deserialize;
use serde_derive::Serialize;
use tracing::{debug, info};

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(default)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub captcha: CaptchaConfig,
  pub store: StoreConfig,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ServerConfig {
  pub host: String,
  pub port: u16,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CaptchaConfig {
  pub length: usize,
  pub expire_seconds: usize,
  pub width: u32,
  pub height: u32,
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
        length: 5,
        expire_seconds: 120,
        width: 200,
        height: 200,
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
    .merge(Toml::file("Config.toml").nested())
    .merge(Env::prefixed(env_prefix).split("_"));
  let config = figment.extract().expect("配置加载失败");
  config
}
