use crate::app_state::AppState;
use axum::extract::Query;
use axum::{Extension, Json};
use serde_derive::{Deserialize, Serialize};
use tracing::{debug, info};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema, IntoParams)]
pub struct VerifyParams {
  token: String,
  value: String,
}

#[utoipa::path(
  get,
  path = "/captcha/verify",
  params(VerifyParams),
  responses(
        (status = 200, description = "OK")
  )
)]
pub async fn verify_captcha_handler(
  Query(params): Query<VerifyParams>,
  Extension(state): Extension<AppState>,
) -> Json<bool> {
  info!("Verifying captcha{:?}", params);
  debug!("{:?}", params);
  let stored = state.store.get(&params.token).await;
  debug!("{:?}", stored);
  let verified = stored == Some(params.value);
  debug!("{:?}", verified);
  state.store.remove(&params.token).await;
  Json(verified)
}
