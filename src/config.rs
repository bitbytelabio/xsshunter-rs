use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
    pub file: String,
    pub tracing: Tracing,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tracing {
    pub thread_name: bool,
    pub file: bool,
    pub target: bool,
    pub line_number: bool,
    pub pretty: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub log: Log,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mode = std::env::var("DEV_MODE")
            .unwrap_or_default()
            .parse::<bool>()
            .unwrap_or(false);

        let config_dir = std::env::var("CONFIG_DIR").unwrap_or_else(|_| "~/config".into());

        let config_file = if mode {
            "xsshunter.dev.toml".to_string()
        } else {
            "xsshunter.toml".to_string()
        };

        let s = Config::builder()
            .add_source(File::with_name(&format!("{config_dir}/{config_file}")).required(true))
            .build()?;

        s.try_deserialize()
    }
}
