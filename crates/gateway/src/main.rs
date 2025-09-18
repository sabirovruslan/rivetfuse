use tracing::info;

pub(crate) mod configure;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    configure::tracing::new().unwrap();
    info!("Tracing initialized");
}
