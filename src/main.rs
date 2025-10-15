use axum::{
    Router,
    http::{Method, StatusCode},
    response::IntoResponse,
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

    println!("found {} route(s)", routes.len());

    for (route, file_path) in routes {
        println!("  - {}", route);
        let file_path_clone = file_path.clone();
        app = app.route(
            &route,
            get(|| async move {
                let file_path = file_path_clone.clone();
                handle_api_route(file_path).await
            }),
        );
    }

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

/// execute javascript file and handle serialization
async fn handle_api_route(file_path: String) -> impl IntoResponse {
    match execute_js_file(&file_path) {
        Ok(result) => {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&result) {

                let body = json.get("body").unwrap().to_string();

                (StatusCode::OK, body)
            } else {
                // if not json, return result as it is.
                (StatusCode::OK, result)
            }
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{{\"error\": \"{}\"}}", error),
        ),
    }
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
