use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: String,
    pub port: String,
    pub expiration: String,
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
