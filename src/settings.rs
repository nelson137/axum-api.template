use anyhow::{Context, Result};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    #[allow(dead_code)]
    pub environment: String,
    pub log_filter: String,
    pub bind_addr: String,
    #[cfg(feature = "loki")]
    pub loki_url: String,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let environment = std::env::var("APP_ENVIRONMENT");
        let environment = environment.as_deref().unwrap_or("production");
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config/settings.toml"))
            .add_source(
                config::File::with_name(&format!("config/settings.{environment}.toml"))
                    .required(false),
            )
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .context("failed to collect config sources")?
            .try_deserialize()
            .context("failed to deserialize settings")?;
        Ok(settings)
    }
}
