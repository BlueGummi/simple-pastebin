use axum::{
    routing::{get, post},
    response::{Html, IntoResponse},
    Router,
};
use chrono::Local;
use config::Config;
use std::{fs, fs::OpenOptions, io::Write, thread};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;
mod config;
fn declare_config() -> Config {
    let config_content = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    toml::de::from_str(&config_content)
        .expect("Failed to parse config.toml")
}
fn clear_file_after_duration(file_path: &str, duration: Duration) {
    loop {
        let start_time = Instant::now();

        // wait for the specified duration
        thread::sleep(duration);

        // clear the file contents
        if let Err(e) = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
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
    std::thread::spawn(move || {
        clear_file_after_duration(&(format!("{}", config.logname.trim())), total_duration);
    });
    server().await;
}

async fn server() {
    let config = declare_config();
    let router: Router = Router::new()
        .route("/", post(write_to_file))
        .route("/", get(serve_form))
        .route(&(format!("/{}", config.logname.trim())), get(serve_log))
        .route("/clear", post(clear_log)) // clear that log
        .route("/config.toml", get(serve_config));
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.address.trim(), config.port.trim())).await.unwrap();
    println!("Server listening on {}:{}", config.address.trim(), config.port.trim());
    //let listener = tokio::net::TcpListener::bind("localhost:80").await.unwrap();
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
    println!("{}", data);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // make lil bro if lil bro is nonexistent
        .open(format!("{}", config.logname.trim()))
        .unwrap();
    
    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("couldn't write to file: {}", e);
    }
}

async fn clear_log() -> impl IntoResponse {
    let config = declare_config();
    // Clear the contents of the log file
    if let Err(e) = fs::write(format!("{}", config.logname.trim()), "") {
        eprintln!("Error clearing log file: {}", e);
        return "Error clearing log file".into_response(); // error message
    }
    println!("Log file cleared");
    "Log file cleared".into_response() // success message
}

async fn serve_form() -> Html<String> {
    let content = fs::read_to_string("index.html").unwrap_or_else(|_| {
        "Error loading HTML file".to_string()
    });
    Html(content)
}

// serve the log file 
async fn serve_log() -> impl IntoResponse {
    let config = declare_config();
    match fs::read_to_string(format!("{}", config.logname.trim())) {
        Ok(content) => content,
        Err(_) => "Error reading log file".to_string(),
    }
}

// this will give the frontend the config file
async fn serve_config() -> impl IntoResponse {
    match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(_) => "Error reading config.toml".to_string(),
    }
}
