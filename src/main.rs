use axum::{
    Router,
    body::Body,
    extract::Query,
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::js_runtime::{JsRequest, execute_js_file};

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
            &route.clone(),
            get(move |headers, query, body| {
                let file_path = file_path_clone.clone();
                async move {
                    handle_api_route(headers, Method::GET, query, body, file_path, route).await
                }
            }),
        );
    }

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// async fn handle_request(app route: &str, method: RequestMethod) {}

/// execute javascript file and handle serialization
async fn handle_api_route(
    headers: HeaderMap,
    method: Method,
    Query(query): Query<HashMap<String, String>>,
    body: String,
    file_path: String,
    route_path: String,
) -> impl IntoResponse {
    let headers_map: HashMap<String, String> = headers
        .iter()
        .filter_map(|(key, value)| {
            value
                .to_str()
                .ok()
                .map(|v| (key.to_string(), v.to_string()))
        })
        .collect();

    let js_request = JsRequest {
        url: route_path,
        headers: headers_map,
        method: method.to_string(),
        query,
        body: if body.is_empty() { None } else { Some(body) },
        params: HashMap::new(),
    };

    match execute_js_file(&file_path, js_request).await {
        Ok(response) => (StatusCode::from_u16(response.status).unwrap_or(StatusCode::OK), serde_json::to_string(&response.body).unwrap()),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{{\"error\": \"{}\"}}", error),
        ),
    }
}

/// Scan the api directory and return tuple of route names and paths to .js files
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
