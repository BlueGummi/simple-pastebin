use crate::*;
use axum::response::IntoResponse;
use log::{error, info};
use std::fs;
pub async fn clear_log() -> impl IntoResponse {
    let config = declare_config();
    match fs::write(config.log_name.as_ref().unwrap().trim(), "") {
        Ok(_) => {
            info!("{}", "Log file cleared.");
            if config.history.unwrap_or(false) {
                write_to_history("Log cleared.".to_string()).await;
            }
            "Log file cleared".into_response()
        }
        Err(e) => {
            error!("Error clearing log file: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error clearing log file",
            )
                .into_response()
        }
    }
}

pub async fn serve_form() -> Html<String> {
    let content = fs::read_to_string("assets/index.html")
        .unwrap_or_else(|_| "Error loading HTML file".to_string());
    Html(content)
}

pub async fn serve_log() -> impl IntoResponse {
    let config = declare_config();
    match fs::read_to_string(config.log_name.as_ref().unwrap().trim()) {
        Ok(content) => content,
        Err(_) => "Error reading log file".to_string(),
    }
}

pub async fn serve_config() -> impl IntoResponse {
    let config = declare_config();
    format!(
        "{}\n{}",
        config.expiration.unwrap(),
        config.log_name.unwrap()
    )
}
