use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use chrono::Local;
use config::Config;
use std::time::{Duration, Instant};
use std::{fs, fs::OpenOptions, io::Write};
use tokio::{time, signal};
mod config;

pub fn declare_config() -> Config {
    let config_content = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(_) => {
            return Config::default();
        }
    };

    toml::de::from_str::<Config>(&config_content).unwrap_or_default()
}

async fn clear_file_after_duration(file_path: &str, duration: Duration) {
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
        } else {
            println!("File cleared after {:?}", start_time.elapsed());
        }
    }
}

#[tokio::main]
async fn main() {
    let config = declare_config();
    let total_duration = Duration::from_secs(config::parse_duration(&config.expiration));

    // spawn a task to clear the file after the specified duration
    let log_name = config.log_name.trim().to_string();
    tokio::spawn(async move {
        clear_file_after_duration(&log_name, total_duration).await;
    });

    server().await;
}

async fn server() {
    let config = declare_config();
    let router: Router = Router::new()
        .route("/", post(write_to_file))
        .route("/", get(serve_form))
        .route(&(format!("/{}", config.log_name.trim())), get(serve_log))
        .route("/clear", post(clear_log))
        .route("/config.toml", get(serve_config));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.address.trim(),
        config.port.trim()
    ))
    .await
    .unwrap();

    if config.display_info == "true" {
        println!(
            "Server listening on {}:{}",
            config.address.trim(),
            config.port.trim()
        );
    }

    println!("Press Ctrl-C to exit.");
    println!("File automatically clears after {}", config.expiration);

    // Start the server
    let server_task = tokio::spawn({
        async move {
            axum::serve(listener, router).await.unwrap();
        }
    });

    // Wait for the shutdown signal
    signal::ctrl_c().await.expect("failed to listen for event");
    println!("Shutting down...");
    server_task.abort();
    let _ = server_task.await;
}

async fn write_to_file(body: String) {
    let config = declare_config();
    let data = format!("{} |: {}", Local::now().format("%D %I:%M:%S %p"), body);
    if config.display_data == "true" {
        println!("{}", data);
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.log_name.trim())
        .unwrap();

    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

async fn clear_log() -> impl IntoResponse {
    let config = declare_config();
    if let Err(e) = fs::write(config.log_name.trim(), "") {
        eprintln!("Error clearing log file: {}", e);
        return "Error clearing log file".into_response();
    }
    println!("Log file cleared");
    "Log file cleared".into_response()
}

async fn serve_form() -> Html<String> {
    let content =
        fs::read_to_string("index.html").unwrap_or_else(|_| "Error loading HTML file".to_string());
    Html(content)
}

async fn serve_log() -> impl IntoResponse {
    let config = declare_config();
    match fs::read_to_string(config.log_name.trim()) {
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
