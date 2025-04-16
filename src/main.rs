pub mod breathes;
use crate::breathes::search_breathes;
use axum::{Router, routing::get};
use serde::Serialize;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub async fn conn() -> Result<Client, Error> {
    assert!(dotenv::from_filename("/etc/qwa/db.env").is_ok());
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

#[derive(Serialize)]
pub struct Breath {
    id: String,
    origin: String,
    content: String,
    timestamp: u64,
    depth: u32,
    tags: Vec<String>,
    status: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/breathes/search/{term}", get(search_breathes));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4213").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Welcome to Kōgnitara"
}
