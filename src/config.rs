use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: String,
    pub port: String,
    pub expiration: String,
    pub log_name: String,
    pub display_data: String,
    pub display_info: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            address: String::from("127.0.0.1"),
            port: String::from("6060"),
            expiration: String::from("10m"),
            log_name: String::from("input.log"),
            display_data: String::from("true"),
            display_info: String::from("true"),
        }
    }
}
pub fn parse_duration(expiration: &str) -> u64 {
    let mut total_seconds = 0;
    let mut number = String::new();
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
    // handle any remaining number at the end (in case of seconds)
    if let Ok(seconds) = number.parse::<u64>() {
        total_seconds += seconds;
    }
    total_seconds
}
