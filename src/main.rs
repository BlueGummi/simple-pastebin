use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use chrono::Local;
use config::Config;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fs, fs::OpenOptions, io::Write, fs::create_dir_all};
use tokio::{signal, time};
mod config;
use owo_colors::OwoColorize;
use tower_http::services::ServeDir;

pub fn declare_config() -> Config {
    let config_content = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        eprintln!("Failed to read config.toml. Using default configuration.");
        String::new()
    });

    let mut config: Config = toml::de::from_str(&config_content).unwrap_or_default();

    config.address.get_or_insert_with(|| "127.0.0.1".to_string());
    config.port.get_or_insert(6060);
    config.expiration.get_or_insert("10m".to_string());
    config.log_name.get_or_insert("logs/input.log".to_string());

    config
}

async fn clear_file_after_duration(file_path: &str, duration: Duration) {
    let config = declare_config();
    loop {
        let start_time = Instant::now();
        time::sleep(duration).await;

        if let Err(e) = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .and_then(|mut file| file.write_all(b""))
        {
            eprintln!("Failed to clear file: {}", e);
        } else if config.display_info.unwrap_or(false) {
            println!(
                "{} {} {:?}",
                "File cleared".green(),
                "after".blue(),
                start_time.elapsed().yellow()
            );
        }
    }
}

#[tokio::main]
async fn main() {
    let config = declare_config();
    let total_duration = config::parse_duration(&config.expiration);

    // Spawn a task to clear the file after the specified duration
    let log_name = config.log_name.expect("log_name issue").trim().to_string();
    tokio::spawn(async move {
        clear_file_after_duration(&log_name, Duration::from_secs(total_duration)).await;
    });

    server().await;
}

async fn server() {
    let config = declare_config();
    let mut router = Router::new()
        .route("/", get(serve_form))
        .route("/clear", post(clear_log))
        .route("/config.toml", get(serve_config))
        .nest_service("/assets", ServeDir::new("assets"));

    if config.void_mode.unwrap_or(false) {
        router = router.route(&(format!("/{}", config.log_name.as_ref().unwrap().trim())), get(serve_log));
        router = router.route("/", post(write_to_log));
    }

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.address.as_ref().unwrap().trim(),
        config.port.unwrap()
    ))
    .await
    .unwrap();

    if config.display_info.unwrap_or(false) {
        println!(
            "{} {} {}:{}",
            "Server listening".green(),
            "on".blue(),
            config.address.as_ref().unwrap().trim().yellow(),
            config.port.unwrap().yellow()
        );
    }
    println!("{}", "Press Ctrl-C to exit.".red());
    println!(
        "{} {} {}",
        "File automatically clears".green(),
        "after".blue(),
        config.expiration.unwrap().trim().yellow()
    );
    let server_task = tokio::spawn({
        async move {
            axum::serve(listener, router).await.unwrap();
        }
    });

    // Wait for the shutdown signal
    signal::ctrl_c().await.expect("failed to listen for event");
    println!("{}", "Shutting down...".red());
    if config.history.unwrap_or(false) {
        write_to_history("Shutdown".to_string()).await;
    }
    server_task.abort();
    let _ = server_task.await;
}

async fn write_to_log(body: String) -> impl IntoResponse {
    let config = declare_config();
    let data = format!("{} |: {}", Local::now().format("%D %I:%M:%S %p"), body);

    if config.display_data.unwrap_or(false) {
        println!("{}", data.white());
    }

    // Create parent directories if they do not exist
    let log_path = Path::new(config.log_name.as_ref().unwrap().trim());
    if let Some(parent) = log_path.parent() {
        if !parent.exists() {
            if let Err(e) = create_dir_all(parent) {
                eprintln!("Couldn't create directories: {}", e);
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
        Ok(_) => "Data written to file".into_response(),
        Err(e) => {
            eprintln!("Couldn't write to file: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error writing to file",
            )
            .into_response()
        }
    }
}

async fn clear_log() -> impl IntoResponse {
    let config = declare_config();
    match fs::write(config.log_name.as_ref().unwrap().trim(), "") {
        Ok(_) => {
            println!("{}", "Log file cleared.".red());
            if config.history.unwrap_or(false) {
                write_to_history("Log cleared.".to_string()).await;
            }
            "Log file cleared".into_response()
        }
        Err(e) => {
            eprintln!("Error clearing log file: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Error clearing log file",
            )
            .into_response()
        }
    }
}

async fn serve_form() -> Html<String> {
    let content = fs::read_to_string("assets/index.html")
        .unwrap_or_else(|_| "Error loading HTML file".to_string());
    Html(content)
}

async fn serve_log() -> impl IntoResponse {
    let config = declare_config();
    match fs::read_to_string(config.log_name.as_ref().unwrap().trim()) {
        Ok(content) => content,
        Err(_) => "Error reading log file".to_string(),
    }
}

async fn serve_config() -> impl IntoResponse {
    match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(_) => "Error reading config.toml".to_string(),
    }
}

async fn write_to_history(mut data: String) {
    let config = declare_config();
    data = format!(
        "Event at {} |: {}",
        Local::now().format("%D %I:%M:%S %p"),
        data
    );

    // Create parent directories if they do not exist
    let history_log_path = Path::new(config.history_log.as_ref().unwrap().trim());
    if let Some(parent) = history_log_path.parent() {
        if !parent.exists() {
            if let Err(e) = create_dir_all(parent) {
                eprintln!("Couldn't create directories: {}", e);
                return; // Exit the function if directory creation fails
            }
        }
    }

    // Open the history log file and write to it
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(history_log_path)
        .and_then(|mut file| writeln!(file, "{}", data))
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't write to history: {}", e);
        }
    }
}