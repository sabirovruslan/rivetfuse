use tokio::net::TcpListener;
use tracing::info;

use crate::{
    configure::AppConfig, error::AppResult, router::create_router, server::state::AppState,
};

pub mod di;
pub mod state;

pub struct AppServer {
    pub state: AppState,
    tcp: TcpListener,
}

impl AppServer {
    pub async fn new(mut config: AppConfig) -> AppResult<Self> {
        let tcp = TcpListener::bind(config.server.socket_address()?).await?;
        let addres = tcp.local_addr()?;
        info!("Server initialized at {addres}");

        config.server.port = addres.port();
        let state = AppState::new(config).await?;

        Ok(Self { state, tcp })
    }

    pub async fn run(self) -> AppResult<()> {
        let router = create_router(self.state);
        axum::serve(self.tcp, router).await?;
        Ok(())
    }
}
