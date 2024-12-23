use crate::declare_config;
use axum::{extract::Path, response::Html, response::IntoResponse};
use colored::Colorize;
use log::{info, warn};
use regex::Regex;
use rusqlite::{params, Connection, OptionalExtension, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Paste {
    id: i64,
    content: String,
}

async fn create_paste(content: String) -> Result<i64> {
    let conn = Connection::open("pastes.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS pastes (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT NOT NULL)", [])?;

    conn.execute("INSERT INTO pastes (content) VALUES (?1)", params![content])?;
    Ok(conn.last_insert_rowid())
}

async fn get_paste(id: i64) -> Result<Option<Paste>> {
    let conn = Connection::open("pastes.db")?;
    let mut stmt = conn.prepare("SELECT id, content FROM pastes WHERE id = ?1")?;
    let paste = stmt
        .query_row(params![id], |row| {
            Ok(Paste {
                id: row.get(0)?,
                content: row.get(1)?,
            })
        })
        .optional()?;
    Ok(paste)
}
pub async fn serve_raw(Path(id): Path<i64>) -> impl IntoResponse {
    match get_paste(id).await {
        Ok(Some(paste)) => Html(paste.content),
        Ok(None) => Html("The requested paste does not exist".to_string()),
        Err(_) => Html("Could not retrieve the paste.".to_string()),
    }
}
pub async fn create_new_paste(content: String) -> impl IntoResponse {
    let config = declare_config();
    match create_paste(content).await {
        Ok(id) => {
            info!(
                "Paste {} {}",
                id.to_string().bold().blue(),
                "created.".bold().green()
            );
            let link = format!(
                "http://{}:{}/{}",
                config.address.unwrap(),
                config.port.unwrap(),
                id
            );
            let response_message = format!("Paste successful, view it at {}", link);
            Html(response_message)
        }
        Err(_) => Html("Failed to create paste".to_string()),
    }
}
pub async fn delete_paste(Path(id): Path<i64>) -> impl IntoResponse {
    let conn = match Connection::open("pastes.db") {
        Ok(conn) => conn,
        Err(_) => return Html("Database connection failed.".to_string()),
    };

    let result = match conn.execute("DELETE FROM pastes WHERE id = ?1", params![id]) {
        Ok(count) => count,
        Err(_) => return Html("Failed to delete paste.".to_string()),
    };

    if result > 0 {
        info!(
            "Paste {} {}",
            id.to_string().bold().blue(),
            "deleted.".bold().red()
        );
        Html(format!("Paste {} deleted.", id).to_string())
    } else {
        Html("Paste not found.".to_string())
    }
}
pub async fn serve_paste(Path(id): Path<i64>) -> impl IntoResponse {
    match get_paste(id).await {
        Ok(Some(paste)) => Html(render_paste_template(&paste.id, &paste.content)),
        Ok(None) => {
            warn!("Client requested paste {} {}", id, " not found");
            Html("<h1>404 Not Found</h1><p>The requested paste does not exist.</p>".to_string())
        }
        Err(_) => Html("<h1>Error</h1><p>Could not retrieve the paste.</p>".to_string()),
    }
}

fn render_paste_template(paste_id: &i64, paste_content: &str) -> String {
    let template = fs::read_to_string("assets/paste.html")
        .unwrap_or_else(|_| "<h1>Error</h1><p>Could not load the paste template.</p>".to_string());
    let linked_content = convert_urls_to_links(&escape_html(paste_content));

    template
        .replace("<span id=\"paste-id\"></span>", &paste_id.to_string())
        .replace(
            "<div class=\"data\" id=\"fileContent\" aria-live=\"polite\"></div>",
            &format!(
                "<div class=\"data\" id=\"fileContent\" aria-live=\"polite\">{}</div>",
                linked_content
            ),
        )
}
fn convert_urls_to_links(text: &str) -> String {
    let url_regex = Regex::new(r"(https?://[^\s]+)").unwrap();
    url_regex
        .replace_all(text, |caps: &regex::Captures| {
            format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                &caps[0], &caps[0]
            )
        })
        .to_string()
}
fn escape_html(content: &str) -> String {
    content
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
