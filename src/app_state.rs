use crate::config::CaptchaConfig;
use crate::store::CaptchaStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
  pub store: Arc<dyn CaptchaStore>,
  pub captcha_config: CaptchaConfig,
}
