use axum::{
    routing::{get, post},
    response::{Html, IntoResponse},
    Router,
};
use chrono::Local;
use config::Config;
use std::{fs, fs::OpenOptions, io::Write};
use std::time::{Duration, Instant};
use tokio::{sync::mpsc, task, time};

mod config;

fn declare_config() -> Config {
    let config_content = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    toml::de::from_str(&config_content)
        .expect("Failed to parse config.toml")
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

    axum::serve(listener, router).await.unwrap();

    let (_, mut rx) = mpsc::unbounded_channel();
    task::spawn(async move {
        while let Some(body) = rx.recv().await {
            write_to_file(body).await;
        }
    });
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
