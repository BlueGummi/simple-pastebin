use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use chrono::Local;
use config::Config;
use std::time::{Duration, Instant};
use std::{fs, fs::OpenOptions, io::Write};
use tokio::{signal, time};
mod config;
use owo_colors::OwoColorize;
use tower_http::services::ServeDir;

pub fn declare_config() -> Config {
    let config_content = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(_) => {
            return Config::default();
        }
    };

    // Deserialize the TOML content into a Config struct
    let mut config: Config = toml::de::from_str::<Config>(&config_content).unwrap_or_default();

    // Check each field and replace with default if empty
    if config.address.is_empty() {
        config.address = String::from("127.0.0.1");
    }
    if config.port == 0 {
        config.port = 6060;
    }
    if config.expiration.is_empty() {
        config.expiration = String::from("10m");
    }
    if config.log_name.is_empty() {
        config.log_name = String::from("input.log");
    }

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
        } else if config.display_info {
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
    let mut router = Router::new()
        .route("/", post(write_to_log))
        .route("/", get(serve_form))
        .route("/clear", post(clear_log))
        .route("/config.toml", get(serve_config))
        .nest_service("/assets", ServeDir::new("assets"));

    if !config.void_mode {
        router = router.route(&(format!("/{}", config.log_name.trim())), get(serve_log));
    }

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.address.trim(), config.port))
            .await
            .unwrap();

    if config.display_info {
        println!(
            "{} {} {}:{}",
            "Server listening".green(),
            "on".blue(),
            config.address.trim().yellow(),
            config.port.yellow()
        );
    }
    println!("{}", "Press Ctrl-C to exit.".red());
    println!(
        "{} {} {}",
        "File automatically clears".green(),
        "after".blue(),
        config.expiration.yellow()
    );
    let server_task = tokio::spawn({
        async move {
            axum::serve(listener, router).await.unwrap();
        }
    });

    // Wait for the shutdown signal
    signal::ctrl_c().await.expect("failed to listen for event");
    println!("{}", "Shutting down...".red());
    if config.history {
        write_to_history("Shutdown".to_string()).await;
    }
    server_task.abort();
    let _ = server_task.await;
}

async fn write_to_log(body: String) -> impl IntoResponse {
    let config = declare_config();
    let data = format!("{} |: {}", Local::now().format("%D %I:%M:%S %p"), body);

    if config.display_data {
        println!("{}", data.white());
    }

    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.log_name.trim())
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
    match fs::write(config.log_name.trim(), "") {
        Ok(_) => {
            println!("{}", "Log file cleared.".red());
            if config.history {
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

async fn write_to_history(mut data: String) {
    let config = declare_config();
    data = format!(
        "Event at {} |: {}",
        Local::now().format("%D %I:%M:%S %p"),
        data
    );
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.history_log.trim())
        .and_then(|mut file| writeln!(file, "{}", data))
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't write to history: {}", e);
        }
    }
}
