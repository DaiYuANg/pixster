use crate::app_state::AppState;
use axum::Extension;
use axum::extract::Query;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use base64::Engine;
use image::codecs::png::PngEncoder;
pub use image::{ColorType, EncodableLayout, ExtendedColorType, ImageEncoder, Luma};
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use serde_derive::Deserialize;
use tracing::debug;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams, Clone)]
pub struct QrRequest {
  text: String,
  #[serde(default)]
  ec_level: Option<String>, // L, M, Q, H
  #[serde(default)]
  size: Option<u32>, // 图片尺寸(px)
  #[serde(default)]
  format: Option<String>, // png, svg, base64_png
}

fn parse_ec_level(level: &Option<String>) -> EcLevel {
  match level.as_deref() {
    Some("L") => EcLevel::L,
    Some("M") | None => EcLevel::M,
    Some("Q") => EcLevel::Q,
    Some("H") => EcLevel::H,
    Some(_) => EcLevel::M,
  }
}

#[utoipa::path(
  get,
  path = "/qrcode",
  params(QrRequest),
  responses(
        (status = 200, description = "OK")
  )
)]
pub async fn generate_qr(
  Query(params): Query<QrRequest>,
  state: Extension<AppState>,
) -> impl IntoResponse {
  if params.text.is_empty() {
    return (
      StatusCode::BAD_REQUEST,
      "Query param `text` cannot be empty",
    )
      .into_response();
  }

  // 生成缓存 key，比如简单拼接后做哈希
  let cache_key = {
    use sha2::{Digest, Sha256};
    let raw_key = format!(
      "{}|{:?}|{:?}|{:?}",
      params.text, params.ec_level, params.size, params.format
    );
    let hash = Sha256::digest(raw_key.as_bytes());
    format!("qr_cache:{:x}", hash)
  };

  debug!("cache key{}", cache_key);

  let ec_level = parse_ec_level(&params.ec_level);
  let size = params.size.unwrap_or(256);
  let format = params.format.as_deref().unwrap_or("png");

  // 生成二维码
  let code = match QrCode::with_error_correction_level(params.text.as_bytes(), ec_level) {
    Ok(c) => c,
    Err(_) => return (StatusCode::BAD_REQUEST, "Failed to generate QR code").into_response(),
  };

  match format {
    "png" | "base64_png" => {
      let image = code.render::<Luma<u8>>().min_dimensions(size, size).build();
      let mut buf = Vec::new();
      let encoder = PngEncoder::new(&mut buf);
      if encoder
        .write_image(
          &image,
          image.width(),
          image.height(),
          ExtendedColorType::from(ColorType::L8),
        )
        .is_err()
      {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode PNG").into_response();
      }

      if format == "png" {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("image/png"));
        (headers, buf).into_response()
      } else {
        // base64_png 返回 base64 字符串
        let b64 = base64::engine::general_purpose::STANDARD.encode(&buf);
        let body = format!("data:image/png;base64,{}", b64);
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("text/plain"));
        (headers, body).into_response()
      }
    }
    "svg" => {
      let svg_string = code
        .render()
        .min_dimensions(256, 256)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

      let mut headers = HeaderMap::new();
      headers.insert("Content-Type", HeaderValue::from_static("image/svg+xml"));

      (headers, svg_string).into_response()
    }
    _ => (
      StatusCode::BAD_REQUEST,
      "Unsupported format: use png, svg, or base64_png",
    )
      .into_response(),
  }
}
