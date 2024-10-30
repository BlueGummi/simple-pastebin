use crate::declare_config;
use axum::{extract::Path, response::Html, response::IntoResponse};
use log::info;
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
pub async fn create_new_paste(content: String) -> impl IntoResponse {
    let config = declare_config();
    match create_paste(content).await {
        Ok(id) => {
            info!("Paste number {} created.", id);
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

pub async fn serve_paste(Path(id): Path<i64>) -> impl IntoResponse {
    match get_paste(id).await {
        Ok(Some(paste)) => {
            let response_html = render_paste_template(&paste.id, &paste.content);
            Html(response_html)
        }
        Ok(None) => {
            Html("<h1>404 Not Found</h1><p>The requested paste does not exist.</p>".to_string())
        }
        Err(_) => Html("<h1>Error</h1><p>Could not retrieve the paste.</p>".to_string()),
    }
}

fn render_paste_template(paste_id: &i64, paste_content: &str) -> String {
    let template = fs::read_to_string("assets/paste.html")
        .unwrap_or_else(|_| "<h1>Error</h1><p>Could not load the paste template.</p>".to_string());
    let escaped_content = escape_html(paste_content);
    let linked_content = convert_urls_to_links(&escaped_content);

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
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}
