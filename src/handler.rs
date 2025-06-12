use crate::app_state::AppState;
use axum::{extract::Extension, response::Json};
use captcha_rs::CaptchaBuilder;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CaptchaResponse {
  token: String,
  captcha_value: String,
  captcha: String,
}

pub async fn generate_captcha_handler(state: Extension<AppState>) -> Json<CaptchaResponse> {
  let token = Uuid::new_v4().to_string();
  let captcha_value = "abcde".to_string();
  let captcha = CaptchaBuilder::new()
    .length(5)
    .width(130)
    .height(40)
    .dark_mode(false)
    .complexity(1) // min: 1, max: 10
    .compression(40) // min: 1, max: 99
    .build()
    .to_base64();
  // 存储验证码
  state
    .store
    .set(token.clone(), captcha_value.clone(), 300)
    .await;

  Json(CaptchaResponse {
    token,
    captcha_value,
    captcha,
  })
}

pub async fn verify_captcha_handler(Extension(state): Extension<AppState>) -> Json<bool> {
  let stored = state.store.get("some-token").await;
  let verified = stored == Some("abcde".to_string());

  Json(verified)
}
