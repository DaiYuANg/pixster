mod app_state;
mod base64_handler;
mod captcha_builder;
mod config;
mod image_handler;
mod memory_store;
mod random;
mod redis_store;
mod store;
mod verify_handler;
use crate::app_state::AppState;
use crate::base64_handler::__path_generate_captcha_handler;
use crate::base64_handler::generate_captcha_handler;
use crate::config::{load_config, AppConfig};
use crate::image_handler::__path_captcha_image_handler;
use crate::image_handler::captcha_image_handler;
use crate::store::create_store;
use crate::verify_handler::__path_verify_captcha_handler;
use crate::verify_handler::verify_captcha_handler;
use axum::{routing::get, Extension, Router};
use axum_prometheus::PrometheusMetricLayer;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const CUSTOMER_TAG: &str = "customer";
const ORDER_TAG: &str = "order";
#[derive(OpenApi)]
#[openapi(
  paths(
    generate_captcha_handler,
    captcha_image_handler,
    verify_captcha_handler,
  ),
  tags(
        (name = CUSTOMER_TAG, description = "Customer API endpoints"),
        (name = ORDER_TAG, description = "Order API endpoints")
  )
)]
struct ApiDoc;

fn init() -> AppConfig {
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
    .with_env_filter(EnvFilter::from_default_env().add_directive("debug".parse().unwrap()))
    .with_ansi(true)
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
  let store = create_store(store_config, captcha_config).await;
  let app_state = AppState { store };
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
    .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
    .layer(prom_layer)
    .layer(TraceLayer::new_for_http())
    .layer(Extension(app_state));
  let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
  info!("Server start at http://{}", addr);
  info!(
    "Captcha API server listening on http://localhost:{}/swagger-ui",
    config.server.port
  );
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}
