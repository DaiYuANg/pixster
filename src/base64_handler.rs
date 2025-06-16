use crate::app_state::AppState;
use crate::captcha_builder::CaptchaParameter;
use axum::extract::Query;
use axum::{Extension, Json};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
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
pub async fn generate_captcha_handler(
  state: Extension<AppState>,
  Query(params): Query<CaptchaParameter>,
) -> Json<CaptchaResponse> {
  let token = Uuid::now_v7().to_string();
  let length = params.length.unwrap_or(5);
  let captcha_value = crate::random::random_string(length);
  let captcha = params.build(captcha_value.clone());
  let base64 = captcha.to_base64();
  state.store.set(token.clone(), captcha_value).await;

  Json(CaptchaResponse {
    token,
    captcha: base64,
  })
}
