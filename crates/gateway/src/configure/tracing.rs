use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

fn create_subscriber(name: &str, env_fileter: EnvFilter) -> impl Subscriber + Sync + Send {
    Registry::default()
        .with(env_fileter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(name.into(), std::io::stdout))
}

pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = create_subscriber("rivertfuse-gateway", EnvFilter::from_default_env());
    LogTracer::init()?;
    set_global_default(subscriber)?;
    Ok(())
}
