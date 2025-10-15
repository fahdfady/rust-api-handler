use axum::{
    Router,
    http::Method,
    routing::{MethodRouter, get},
};
use std::path::PathBuf;

use crate::js_runtime::execute_js_file;

mod js_runtime;

#[tokio::main]
async fn main() {
    println!("🚀 Starting server on http://localhost:3000");
    let mut app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let routes = scan_api_dir("api");

    // println!("Routes: {routes:#?}");

    println!("found {} route(s)", routes.len());

    for (route, file_path) in routes {
        println!("  - {}", route);
        app = app.route(
            &route,
            get(|| async move { execute_js_file(&file_path).unwrap() }),
        );
    }

    // Create a simple router with one test route

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

/// Scan the api directory and return pairs of all route names and .js files
fn scan_api_dir(dir: &str) -> Vec<(String, String)> {
    let mut routes: Vec<(String, String)> = Vec::new();

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

                    let file_path = path.to_string_lossy().to_string();

                    routes.push((route, file_path));
                }
            }
        }
    }

    routes
}
