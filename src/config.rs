use clap::Parser;
use colored::Colorize;
use log::warn;
use serde::Deserialize;
use std::fs;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub expiration: Option<String>,
    pub log_name: Option<String>,
    pub void_mode: Option<bool>,
    pub history: Option<bool>,
    pub history_log: Option<String>,
    pub log_level: Option<String>,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            address: None,
            port: None,
            expiration: None,
            log_name: None,
            void_mode: Some(false),
            history: Some(false),
            history_log: None,
            log_level: None,
        }
    }
}
pub fn declare_config() -> Config {
    let config_content = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        warn!(
            "{}",
            "Failed to read config.toml. Using default configuration."
                .bold()
                .yellow()
        );
        String::new()
    });

    let mut config: Config = toml::de::from_str(&config_content).unwrap_or_default();
    let cli = Cli::parse();
    config.address = cli.address.or(config.address);
    config.port = cli.port.or(config.port);
    config.expiration = cli.expiration.or(config.expiration);
    config.log_name = cli.log_name.or(config.log_name);
    config.log_level = cli.log_level.or(config.log_level);
    config.void_mode = cli.void_mode.or(config.void_mode);
    config.history = cli.history.or(config.history);
    config.history_log = cli.history_log.or(config.history_log);
    config.log_level.get_or_insert("info".to_string());
    config.address.get_or_insert_with(|| "0.0.0.0".to_string());
    config.port.get_or_insert(6060);
    config.expiration.get_or_insert("10m".to_string());
    config.log_name.get_or_insert("input.log".to_string());
    std::env::set_var("RUST_LOG", config.log_level.as_ref().unwrap());
    config
}

#[derive(Parser)]
struct Cli {
    /// Address to bind to
    #[arg(short, long)]
    address: Option<String>,

    /// Port to listen on
    #[arg(short, long)]
    port: Option<u16>,

    /// Expiration duration
    #[arg(short, long)]
    expiration: Option<String>,

    /// Log file name
    #[arg(short, long)]
    log_name: Option<String>,

    /// Void mode flag
    #[arg(long)]
    void_mode: Option<bool>,

    /// History flag
    #[arg(long)]
    history: Option<bool>,

    /// History log name
    #[arg(long)]
    history_log: Option<String>,

    /// Set the log level
    #[arg(long)]
    log_level: Option<String>,
}

pub fn parse_duration(expiration: &Option<String>) -> u64 {
    let mut total_seconds = 0;
    let mut number = String::new();
    if let Some(ref expiration) = expiration {
        for c in expiration.chars() {
            match c {
                'd' => {
                    if let Ok(days) = number.parse::<u64>() {
                        total_seconds += days * 86400;
                    }
                    number.clear();
                }
                'h' => {
                    if let Ok(hours) = number.parse::<u64>() {
                        total_seconds += hours * 3600;
                    }
                    number.clear();
                }
                'm' => {
                    if let Ok(minutes) = number.parse::<u64>() {
                        total_seconds += minutes * 60;
                    }
                    number.clear();
                }
                's' => {
                    if let Ok(seconds) = number.parse::<u64>() {
                        total_seconds += seconds;
                    }
                    number.clear();
                }
                _ => {
                    number.push(c);
                }
            }
        }
    }
    // handle any remaining number at the end (in case of seconds)
    if let Ok(seconds) = number.parse::<u64>() {
        total_seconds += seconds;
    }
    total_seconds
}
