use axum::{Extension, Json};
use axum::extract::Query;
use serde_derive::Deserialize;
use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct VerifyParams {
    token: String,
    value: String,
}

pub async fn verify_captcha_handler(
    Query(params): Query<VerifyParams>,
    Extension(state): Extension<AppState>,
) -> Json<bool> {
    let stored = state.store.get(&params.token).await;
    let verified = stored == Some(params.value);

    Json(verified)
}