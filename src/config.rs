use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub expiration: Option<String>,
    pub log_name: Option<String>,
    pub display_data: Option<bool>,
    pub display_info: Option<bool>,
    pub void_mode: Option<bool>,
    pub history: Option<bool>,
    pub history_log: Option<String>,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            address: None,
            port: None,
            expiration: None,
            log_name: None,
            display_data: Some(true),
            display_info: Some(true),
            void_mode: Some(false),
            history: Some(false),
            history_log: None,
        }
    }
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
