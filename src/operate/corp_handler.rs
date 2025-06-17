use axum::body::Bytes;
use axum::extract::Query;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use image::ImageReader;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
struct ImgParams {
  format: Option<String>, // 输出格式 png, jpg, jpeg, webp
  quality: Option<u8>,    // 压缩质量，默认 80（只对 jpeg/webp 生效）
  crop_x: Option<u32>,
  crop_y: Option<u32>,
  crop_w: Option<u32>,
  crop_h: Option<u32>,
}

async fn image_handler(Query(params): Query<ImgParams>, body: Bytes) -> impl IntoResponse {
  let bytes = body.to_vec();

  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", HeaderValue::from_static("plain/text"));

  headers.into_response()
}
