mod app_state;
mod base64_handler;
mod config;
mod image_handler;
mod memory_store;
mod random;
mod redis_store;
mod store;
mod verify_handler;

use crate::app_state::AppState;
use crate::base64_handler::generate_captcha_handler;
use crate::config::{AppConfig, load_config};
use crate::image_handler::captcha_image_handler;
use crate::store::create_store;
use crate::verify_handler::verify_captcha_handler;
use axum::{Extension, Router, routing::get};
use axum_prometheus::PrometheusMetricLayer;
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

const CUSTOMER_TAG: &str = "customer";
const ORDER_TAG: &str = "order";

#[derive(OpenApi)]
#[openapi(
  tags(
        (name = CUSTOMER_TAG, description = "Customer API endpoints"),
        (name = ORDER_TAG, description = "Order API endpoints")
  )
)]
struct ApiDoc;

fn init() -> AppConfig {
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
    .init();
  let config = load_config();
  info!("captcha config: {:?}", config);
  config
}

async fn shutdown_signal() {
  let ctrl = async {
    signal::ctrl_c().await.expect("install ctrl_c");
  };
  #[cfg(unix)]
  let term = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("install SIGTERM")
      .recv()
      .await;
  };
  #[cfg(not(unix))]
  let term = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl => {},
      _ = term => {},
  }
  info!("Shutdown signal received");
}

#[tokio::main]
async fn main() {
  let config = init();
  let store_config = config.clone().store;
  let captcha_config = config.captcha.clone();
  let store = create_store(store_config).await;
  let app_state = AppState {
    store,
    captcha_config,
  };
  let (prom_layer, metric_handle) = PrometheusMetricLayer::pair();

  let app = Router::new()
    .route("/captcha/generate", get(generate_captcha_handler))
    .route("/captcha/generate/image", get(captcha_image_handler))
    .route("/captcha/verify", get(verify_captcha_handler))
    .route("/ping", get(|| async { "pong" }))
    .route(
      "/metrics",
      get(move || async move { metric_handle.render() }),
    )
    .layer(prom_layer)
    .layer(Extension(app_state));
  let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
  info!("Server start at http://{}", addr);
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}
