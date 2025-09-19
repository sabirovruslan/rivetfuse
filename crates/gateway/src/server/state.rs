use std::sync::Arc;

use crate::{configure::AppConfig, error::AppResult, server::di::Container};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub di: Arc<Container>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<AppState> {
        Ok(Self {
            config: Arc::new(config),
            di: Arc::new(Container::new()),
        })
    }
}
