use axum::{Router, routing::get};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    println!("🚀 Starting server on http://localhost:3000");

    let routes = scan_api_dir("api");

    println!("Routes: {routes:#?}");

    // Create a simple router with one test route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

/// Scan the api directory and return all .js files
fn scan_api_dir(dir: &str) -> Vec<String> {
    let mut routes: Vec<String> = Vec::new();

    let path = PathBuf::from(dir);

    if !path.exists() {
        eprint!("Directory {path:?} not found")
    }

    if let Ok(entries) = path.read_dir() {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("js") {
                // get stem. file name without extenstion to add to route
                if let Some(file_name) = path.file_stem() {

                    let route = format!("/api/{}", file_name.to_str().unwrap());
                    routes.push(route);
                }
            }
        }
    }

    routes
}
