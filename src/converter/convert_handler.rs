use axum::body::Bytes;
use axum::extract::{Multipart, Query};
use axum::response::{IntoResponse, Response};
use base64::Engine;
use base64::engine::general_purpose;
use image::ImageFormat;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct ConvertParams {
  #[schema(example = "png")]
  target: Option<String>,
  #[schema(example = "base64")]
  output: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
pub struct ConvertResponse {
  format: String,
  content_type: String,
  data: String,
}

#[derive(utoipa::ToSchema)]
pub struct MultipartUpload {
  /// 上传的图片文件
  #[schema(format = "binary")]
  file: Vec<u8>,
}

#[utoipa::path(
  post,
  path = "/convert",
  tag = "Image",
  params(
        ("target" = String, Query, description = "目标格式"),
        ("output" = Option<String>, Query, description = "输出格式，base64 或 binary")
  ),
  request_body(
        content_type = "multipart/form-data",
        description = "上传的图片文件",
  ),
  responses(
        (status = 200, description = "图片转换成功，base64 JSON 返回", body = ConvertResponse),
        (status = 200, description = "图片转换成功，二进制直接返回", content_type = "image/png")
  )
)]
pub async fn convert_image(
  Query(params): Query<ConvertParams>,
  mut multipart: Multipart,
) -> Response {
  // 读取文件字段
  let mut image_bytes = None;
  while let Some(field) = multipart.next_field().await.unwrap() {
    if field.name() == Some("file") {
      image_bytes = Some(field.bytes().await.unwrap());
      break;
    }
  }

  let Some(image_data) = image_bytes else {
    return (
      axum::http::StatusCode::BAD_REQUEST,
      "Missing file field".to_string(),
    )
      .into_response();
  };

  // 加载图片
  let img = match image::load_from_memory(&image_data) {
    Ok(img) => img,
    Err(_) => return (axum::http::StatusCode::BAD_REQUEST, "Invalid image").into_response(),
  };

  let target = params.target.unwrap_or_else(|| "png".to_string());
  let output = params.output.unwrap_or_else(|| "binary".to_string());

  let format = match target.as_str() {
    "png" => ImageFormat::Png,
    "jpg" | "jpeg" => ImageFormat::Jpeg,
    "webp" => ImageFormat::WebP,
    _ => return (axum::http::StatusCode::BAD_REQUEST, "Unsupported format").into_response(),
  };

  // 输出到 buffer
  let mut out = Cursor::new(Vec::new());
  if let Err(_) = img.write_to(&mut out, format.clone()) {
    return (
      axum::http::StatusCode::INTERNAL_SERVER_ERROR,
      "Convert failed",
    )
      .into_response();
  }
  let out_bytes = out.into_inner();

  match output.as_str() {
    "base64" => {
      let mime = match target.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        _ => "application/octet-stream",
      };

      let base64_str = general_purpose::STANDARD.encode(out_bytes);
      let data_uri = format!("data:{};base64,{}", mime, base64_str);

      axum::Json(serde_json::json!({
          "format": target,
          "content_type": mime,
          "data": data_uri,
      }))
      .into_response()
    }

    _ => {
      let content_type = match target.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        _ => "application/octet-stream",
      };

      (
        axum::http::StatusCode::OK,
        [("Content-Type", content_type)],
        Bytes::from(out_bytes),
      )
        .into_response()
    }
  }
}
