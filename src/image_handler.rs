use crate::app_state::AppState;
use crate::random::random_string;
use axum::Extension;
use axum::body::Body;
use axum::http::{Response, StatusCode, header};
use axum::response::IntoResponse;
use captcha_rs::CaptchaBuilder;
use std::io::Cursor;
use uuid::Uuid;

pub async fn captcha_image_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
  let token = Uuid::now_v7().to_string();
  let length = 5;
  let captcha_value = random_string(length);
  let captcha = CaptchaBuilder::new()
    .length(length)
    .width(130)
    .text(captcha_value.clone())
    .height(40)
    .dark_mode(false)
    .complexity(10) // min: 1, max: 10
    .compression(99) // min: 1, max: 99
    .build();
  let image_bytes = {
    let mut buf = Vec::new();
    captcha
      .image
      .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
      .unwrap();
    buf
  };
  state.store.set(token.clone(), captcha_value.clone()).await;
  // 构造响应
  Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "image/png")
    .header("X-Captcha-Token", token) // 自定义header写token
    .body(Body::from(image_bytes))
    .unwrap()
}
