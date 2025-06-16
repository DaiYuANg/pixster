use crate::app_state::AppState;
use crate::captcha_builder::CaptchaParameter;
use crate::random::random_string;
use axum::body::Body;
use axum::extract::Query;
use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use axum::Extension;
use std::io::Cursor;
use uuid::Uuid;

#[utoipa::path(
  get,
  path = "/captcha/generate/image",
  responses(
        (status = 200, description = "Generate image captcha", content_type = "image/png")
  )
)]
pub async fn captcha_image_handler(
  Extension(state): Extension<AppState>,
  Query(params): Query<CaptchaParameter>,
) -> impl IntoResponse {
  let token = Uuid::now_v7().to_string();
  let length = params.length.unwrap_or(5);
  let captcha_value = random_string(length);
  let captcha = params.build(captcha_value.clone());
  let image_bytes = {
    let mut buf = Vec::new();
    captcha
      .image
      .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
      .unwrap();
    buf
  };
  state.store.set(token.clone(), captcha_value).await;
  // 构造响应
  Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "image/png")
    .header("X-Captcha-Token", token) // 自定义header写token
    .body(Body::from(image_bytes))
    .unwrap()
}
