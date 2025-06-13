use crate::app_state::AppState;
use axum::{Extension, Json};
use captcha_rs::CaptchaBuilder;
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CaptchaResponse {
  token: String,
  captcha: String,
}

#[utoipa::path(
  method(get, head),
  path = "/captcha/generate",
  responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
  )
)]
pub async fn generate_captcha_handler(state: Extension<AppState>) -> Json<CaptchaResponse> {
  let token = Uuid::now_v7().to_string();
  let length = 5;
  let captcha_value = crate::random::random_string(length);
  let captcha = CaptchaBuilder::new()
    .length(length)
    .width(130)
    .text(captcha_value.clone())
    .height(40)
    .dark_mode(false)
    .complexity(1) // min: 1, max: 10
    .compression(1) // min: 1, max: 99
    .build()
    .to_base64();

  state.store.set(token.clone(), captcha_value.clone()).await;

  Json(CaptchaResponse { token, captcha })
}
