use crate::*;
use axum::response::IntoResponse;
use log::{error, info, warn};
use std::{fs, path::Path};
use tokio::fs::{metadata, File};
use tokio::io::{self, AsyncWriteExt};

const MAX_SIZE: u64 = 10 * 1024 * 1024;
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
pub async fn clear_file_if_too_large<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    if let Ok(file_metadata) = metadata(&file_path).await {
        let file_size = file_metadata.len();
        if file_size > MAX_SIZE {
            warn!(
                "File exceeds 10 MB. Clearing the file: {:?}",
                file_path.as_ref()
            );
            let mut file = File::create(&file_path).await?;

            file.write_all(b"File cleared due to size limit exceeded.")
                .await?;

            info!("File cleared successfully.");
        }
    } else {
        warn!("File does not exist. Creating: {:?}", file_path.as_ref());
        let mut file = File::create(&file_path).await?;
        file.write_all(b"")
            .await?;
        info!("File created successfully.");
    }

    Ok(())
}
