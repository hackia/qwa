use crate::conn;
use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Breath {
    id: i32,
    title: String,
    description: String,
    breath: String,
    author: String,
    url: String,
    categories: String,
    tags: String,
    history: String,
    properties: String,
    depth: i32,
    stars: i32,
    status: String,
    created_at: String,
    updated_at: String,
}

pub async fn search_breathes(Path(q): Path<String>) -> impl IntoResponse {
    let client = match conn().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]));
        }
    };

    let x = format!(
        "SELECT * FROM breathes WHERE title LIKE '%{q}%' OR description LIKE '%{q}%' OR author LIKE '%{q}%' OR tags LIKE '%{q}%' OR categories LIKE '%{q}%'",
    );

    let rows = match client.query(x.as_str(), &[]).await {
        Ok(r) => r,
        Err(e) => {
            println!("Error {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]));
        }
    };
    let mut data: Vec<Breath> = Vec::with_capacity(rows.len());
    for row in &rows {
        data.push(Breath {
            id: row.try_get("id").expect("Error getting Breath id"),
            title: row.try_get("title").expect("Error getting Breath title"),
            description: row
                .try_get("description")
                .expect("Error getting Breath description"),
            breath: row.try_get("breath").expect("Error getting Breath"),
            author: row.try_get("author").expect("Error getting Breath author"),
            url: row.try_get("url").expect("Error getting Breath url"),
            categories: row
                .try_get("categories")
                .expect("Error getting Breath categories"),
            tags: row.try_get("tags").expect("Error getting Breath tags"),
            history: row
                .try_get("history")
                .expect("Error getting Breath history"),
            properties: row
                .try_get("properties")
                .expect("Error getting Breath properties"),
            depth: row.try_get("depth").expect("Error getting Breath depth"),
            stars: row.try_get("stars").expect("Error getting Breath stars"),
            status: row.try_get("status").expect("Error getting Breath status"),
            created_at: row
                .try_get("created_at")
                .expect("Error getting Breath created_at"),
            updated_at: row
                .try_get("updated_at")
                .expect("Error getting Breath updated_at"),
        });
    }
    (StatusCode::OK, Json(data))
}
