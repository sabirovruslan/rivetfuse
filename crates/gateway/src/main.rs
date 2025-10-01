use tracing::info;

use crate::error::AppResult;
use crate::server::AppServer;

pub(crate) mod configure;
pub(crate) mod constant;
pub(crate) mod error;
pub(crate) mod provider;
pub(crate) mod router;
pub(crate) mod server;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();
    configure::tracing::new().unwrap();
    info!("Tracing initialized");

    let config = constant::CONFIG.clone();

    let server = AppServer::new(config).await?;
    server.run().await?;

    Ok(())
}
