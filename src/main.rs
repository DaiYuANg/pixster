mod app_state;
mod config;
mod handler;
mod memory_store;
mod redis_store;
mod store;
mod redb_store;

use crate::app_state::AppState;
use crate::config::{load_config, AppConfig};
use crate::handler::{generate_captcha_handler, verify_captcha_handler};
use crate::store::{create_store, StoreBackend};
use axum::routing::post;
use axum::{
  routing::get,
  Extension,
  Router,
};
use std::net::SocketAddr;
use tracing::{info};

fn init() -> AppConfig {
  tracing_subscriber::fmt::init();
  let config = load_config();
  info!("captcha config: {:?}", config);
  config
}

#[tokio::main]
async fn main() {
  let config = init();
  let store = create_store(StoreBackend::Memory);
  let app_state = AppState { store };
  let app = Router::new()
    .route("/captcha/generate", get(generate_captcha_handler))
    .route("/captcha/verify", post(verify_captcha_handler))
    .layer(Extension(app_state));
  let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
  info!("Server start at http://{}", addr);
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
