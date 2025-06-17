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
  ignore_case: Option<bool>,
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
  info!("Verifying captcha {:?}", params);
  let stored_opt = state.store.get(&params.token).await;

  let verified = match stored_opt {
    Some(stored) => {
      if params.ignore_case.unwrap_or(false) {
        stored.eq_ignore_ascii_case(&params.value)
      } else {
        stored == params.value
      }
    }
    None => false,
  };

  debug!("Verification result: {:?}", verified);
  state.store.remove(&params.token).await;
  Json(verified)
}
