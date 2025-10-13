use std::path::PathBuf;

// src/main.rs
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    println!("🚀 Starting server on http://localhost:3000");

    scan_api_dir("api");
    
    // Create a simple router with one test route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

/// Scan the api directory and return all .js files
fn scan_api_dir(dir: &str) {
    let path = PathBuf::from(dir);

    if !path.exists() {
        eprint!("Directory {path:?} not found")
    }

    if let Ok(entries) = path.read_dir() {
        for entry in entries.flatten() {
            let path = entry.path();
            println!("{:?}", path);
        }
    }
}
