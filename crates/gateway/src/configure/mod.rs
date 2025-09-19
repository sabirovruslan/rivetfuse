use std::str::FromStr;

use config::{ConfigError, Environment};
use serde::Deserialize;

pub mod env;
pub mod tracing;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {}

impl AppConfig {
    pub fn init(env_src: Environment) -> Result<Self, ConfigError> {
        let settings_dir = get_settings_from_dir()?;
        let profile = std::env::var("APP_PROFILE")
            .map(|env| EnvProfile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or(Ok(EnvProfile::Dev))?;

        let config = config::Config::builder()
            .add_source(config::File::from(settings_dir.join("base.toml")))
            .add_source(config::File::from(
                settings_dir.join(format!("{profile}.toml")),
            ))
            .add_source(env_src)
            .build()?;

        config.try_deserialize()
    }
}

fn get_settings_from_dir() -> Result<std::path::PathBuf, ConfigError> {
    let path = common::dir::get_project_root()
        .map_err(|e| ConfigError::Message(format!("Failed to get project root: {}", e)))?
        .join("settings");
    Ok(path)
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    strum::Display,
    strum::EnumString,
    serde::Deserialize,
)]
pub enum EnvProfile {
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}
