use std::sync::LazyLock;

use crate::configure::{AppConfig, env::get_env_source};

pub const ENV_PREFIX: &str = "APP";

pub const CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    let env_src = get_env_source(ENV_PREFIX);
    AppConfig::init(env_src).unwrap()
});
