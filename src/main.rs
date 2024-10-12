use axum::{
    routing::{get, post},
    response::{Html, IntoResponse},
    Router,
};
use chrono::Local;
use config::Config;
use std::{fs, fs::OpenOptions, io::Write};
use std::time::{Duration, Instant};
use tokio::{sync::Notify, time};
use std::sync::Arc;
mod config;

pub fn declare_config() -> Config {
    let config_content = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(_) => {
            return Config::default(); 
        }
    };

    match toml::de::from_str::<Config>(&config_content) {
        Ok(config) => config, 
        Err(_) => {
            Config::default() // return default config if parsing fails
        },
    }
}

async fn clear_file_after_duration(file_path: &str, duration: Duration) {
    loop {
        let start_time = Instant::now();
        time::sleep(duration).await;
        
        if let Err(e) = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .and_then(|mut file| file.write_all(b"")) {
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
    
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.address.trim(), config.port.trim())).await.unwrap();
    if config.display_info == "true" {
        println!("Server listening on {}:{}", config.address.trim(), config.port.trim());
    }
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    println!("Press Ctrl-C to exit.");
    println!("File automatically clears after {}", config.expiration);
    // spawn a task to listen for Ctrl-C
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        notify_clone.notify_one();
    });
    // Start the server
    let server_task = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    // wait for the shutdown signal
    notify.notified().await;
    println!("Shutting down...");
    let _ = server_task.abort();
    let _ = server_task.await;
}

async fn write_to_file(body: String) {
    let config = declare_config();
    let data = format!("{} |: {}", Local::now().format("%D %I:%M:%S %p").to_string(), body);
    if config.display_data == "true" {
    println!("{}", data);
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}", config.log_name.trim()))
        .unwrap();
    
    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

async fn clear_log() -> impl IntoResponse {
    let config = declare_config();
    if let Err(e) = fs::write(format!("{}", config.log_name.trim()), "") {
        eprintln!("Error clearing log file: {}", e);
        return "Error clearing log file".into_response();
    }
    println!("Log file cleared");
    "Log file cleared".into_response()
}

async fn serve_form() -> Html<String> {
    let content = fs::read_to_string("index.html").unwrap_or_else(|_| {
        "Error loading HTML file".to_string()
    });
    Html(content)
}

async fn serve_log() -> impl IntoResponse {
    let config = declare_config();
    match fs::read_to_string(format!("{}", config.log_name.trim())) {
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
