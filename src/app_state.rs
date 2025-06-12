use std::sync::Arc;
use crate::store::CaptchaStore;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn CaptchaStore>,
}