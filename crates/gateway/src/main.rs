use tracing::info;

pub(crate) mod configure;
pub(crate) mod constant;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    configure::tracing::new().unwrap();
    info!("Tracing initialized");
}
