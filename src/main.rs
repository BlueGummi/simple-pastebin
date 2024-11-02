use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use chrono::Local;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fs::create_dir_all, fs::OpenOptions, io::Write};
use tokio::{signal, time};
mod config;
use crate::config::*;
mod database;
use database::*;
mod helpers;
use crate::helpers::*;
use log::{error, info};
use tower_http::services::ServeDir;

async fn clear_file_after_duration(file_path: &str, duration: Duration) {
    loop {
        let start_time = Instant::now();
        time::sleep(duration).await;

        if let Err(e) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .and_then(|mut file| file.write_all(b""))
        {
            error!("Failed to clear file: {}", e);
        } else if start_time.elapsed().as_secs() % 3600 == 0 {
            info!(
                "File cleared after {} hours.",
                start_time.elapsed().as_secs() / 3600
            );
        } else if start_time.elapsed().as_secs() % 60 == 0 {
            info!(
                "File cleared after {} minutes.",
                start_time.elapsed().as_secs() / 60
            );
        } else {
            info!(
                "File cleared after {} seconds.",
                start_time.elapsed().as_secs()
            );
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = declare_config();
    let total_duration = config::parse_duration(&config.expiration);
    info!("Server started.");
    tokio::spawn(async move {
        clear_file_after_duration(
            &declare_config()
                .log_name
                .expect("log_name issue")
                .trim()
                .to_string(),
            Duration::from_secs(total_duration),
        )
        .await;
    });
    tokio::spawn(async move {
        loop {
            clear_file_if_too_large(
                &declare_config()
                    .log_name
                    .expect("log_name issue")
                    .trim()
                    .to_string(),
            )
            .await
            .unwrap_or_else(|err| {
                error!("Failed to clear log file: {:?}", err);
            });
            clear_file_if_too_large("pastes.db")
                .await
                .unwrap_or_else(|err| {
                    error!("Failed to clear database file: {:?}", err);
                });

            time::sleep(Duration::from_secs(5)).await;
        }
    });
    server().await;
}

async fn server() {
    let config = declare_config();
    let mut router = Router::new()
        .route("/", get(serve_form))
        .route("/clear", post(clear_log))
        .route("/config", get(serve_config))
        .route("/new", post(create_new_paste))
        .route("/:id", get(serve_paste))
        .route("/raw/:id", get(serve_raw))
        .route("/:id/delete", post(delete_paste))
        .nest_service("/assets", ServeDir::new("assets"));
    if !config.void_mode.unwrap_or(false) {
        router = router.route("/log", get(serve_log));
        router = router.route("/", post(write_to_log));
    }
    match std::net::TcpListener::bind(("127.0.0.1", config.port.unwrap())) {
        Ok(_) => (),
        Err(_) => {
            error!("Port {} cannot be bound!", config.port.unwrap());
            std::process::exit(1);
        }
    }

    match std::net::TcpListener::bind(format!(
        "{}:{}",
        config.address.as_ref().unwrap().trim(),
        config.port.unwrap()
    )) {
        Ok(_) => (),
        Err(_) => {
            error!("Address {} cannot be assigned!", config.address.unwrap());
            std::process::exit(1);
        }
    }
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.address.as_ref().unwrap().trim(),
        config.port.unwrap()
    ))
    .await
    .unwrap();

    info!(
        "{} {} {}:{}",
        "Server listening",
        "on",
        config.address.as_ref().unwrap().trim(),
        config.port.unwrap()
    );
    info!("Main log at {}", config.log_name.as_ref().unwrap());
    info!("Press Ctrl-C to exit.");
    info!(
        "File automatically clears after {}",
        config.expiration.unwrap().trim()
    );
    let server_task = tokio::spawn({
        async move {
            axum::serve(listener, router).await.unwrap();
        }
    });

    // Wait for the shutdown signal
    signal::ctrl_c().await.expect("failed to listen for event");
    info!("Shutting down...");
    if config.history.unwrap_or(false) {
        write_to_history("Shutdown".to_string()).await;
    }
    server_task.abort();
    let _ = server_task.await;
}

async fn write_to_log(body: String) -> impl IntoResponse {
    let config = declare_config();
    let data = format!("{} |: {}", Local::now().format("%D %I:%M:%S %p"), body);
    info!("{}", data);
    // Create parent directories if they do not exist
    let log_path = Path::new(config.log_name.as_ref().unwrap().trim());
    if let Some(parent) = log_path.parent() {
        if !parent.exists() {
            if let Err(e) = create_dir_all(parent) {
                error!("Couldn't create directories: {}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating directories",
                )
                    .into_response();
            }
        }
    }

    // Open the log file and write to it
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path)
        .and_then(|mut file| writeln!(file, "{}", data))
    {
        Ok(_) => "Data written to main pastebin.".into_response(),
        Err(e) => {
            error!("Couldn't write to file: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error writing to file",
            )
                .into_response()
        }
    }
}

async fn write_to_history(mut data: String) {
    let config = declare_config();
    data = format!(
        "Event at {} |: {}",
        Local::now().format("%D %I:%M:%S %p"),
        data
    );

    let history_log_path = Path::new(config.history_log.as_ref().unwrap().trim());
    if let Some(parent) = history_log_path.parent() {
        if !parent.exists() {
            if let Err(e) = create_dir_all(parent) {
                error!("Couldn't create directories: {}", e);
                return;
            }
        }
    }

    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(history_log_path)
        .and_then(|mut file| writeln!(file, "{}", data))
    {
        Ok(_) => {}
        Err(e) => {
            error!("Couldn't write to history: {}", e);
        }
    }
}
