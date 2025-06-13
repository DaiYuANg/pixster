use crate::app_state::AppState;
use axum::extract::Query;
use axum::{Extension, Json};
use serde_derive::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct VerifyParams {
  token: String,
  value: String,
}

pub async fn verify_captcha_handler(
  Query(params): Query<VerifyParams>,
  Extension(state): Extension<AppState>,
) -> Json<bool> {
  info!("Verifying captcha{:?}", params);
  println!("{:?}", params);
  let stored = state.store.get(&params.token).await;
  println!("{:?}", stored);
  let verified = stored == Some(params.value);

  Json(verified)
}
