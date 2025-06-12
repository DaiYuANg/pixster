use dotenvy::dotenv;
use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use serde::Deserialize;
use serde_derive::Serialize;
use tracing::debug;

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(default)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub captcha: CaptchaConfig,
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
    }
  }
}

pub fn load_config() -> AppConfig {
  dotenv().ok();
  std::env::vars()
    .filter(|(k, _)| k.starts_with("CAPSTER_"))
    .for_each(|(k, v)| debug!("{} = {}", k, v));
  let figment = Figment::from(Serialized::defaults(AppConfig::default()))
    .merge(Toml::file("Config.toml").nested())
    .merge(Env::prefixed("CAPSTER_"));
  debug!("{:#?}", figment.find_value("server.port"));
  let config = figment.extract_lossy().expect("配置加载失败");
  config
}
