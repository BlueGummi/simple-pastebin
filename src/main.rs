use axum::{
    Router,
    routing::{post, get},
    response::{Html, IntoResponse},
};
use tokio::{sync::mpsc, task};
use std::{fs, fs::OpenOptions, io::Write};
use chrono::Local;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let router: Router = Router::new()
        .route("/", post(write_to_file))
        .route("/", get(serve_form))
        .route("/input.log", get(serve_log))
        .route("/clear", post(clear_log)); // clear that log

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5050").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    let (_, mut rx) = mpsc::unbounded_channel();
    task::spawn(async move {
        while let Some(body) = rx.recv().await {
            write_to_file(body).await;
        }
    });
}

async fn write_to_file(body: String) {
    let current_time = Local::now();
    let stringtime = current_time.format("%D %I:%M:%S %p").to_string();
    let data = format!("{} |: {}", stringtime, body);
    println!("{}", data);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // make lil bro if lil bro is nonexistent
        .open("input.log")
        .unwrap();
    
    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("couldn't write to file: {}", e);
    }
}

async fn clear_log() -> impl IntoResponse {
    // Clear the contents of the log file
    if let Err(e) = fs::write("input.log", "") {
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
    match fs::read_to_string("input.log") {
        Ok(content) => content,
        Err(_) => "Error reading log file".to_string(),
    }
}