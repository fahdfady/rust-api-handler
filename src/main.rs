use axum::{
    Router,
    extract::Query,
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::{collections::HashMap, path::PathBuf};

use crate::js_runtime::{JsRequest, execute_js_file};

mod js_runtime;

#[tokio::main]
async fn main() {
    println!("🚀 Starting server on http://localhost:3000");
    let mut app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let routes = scan_api_dir("api");

    println!("found {} route(s)", routes.len());

    for (route_path, file_path) in routes {
        println!("  - {}", route_path);
        let fp_get = file_path.clone();
        let fp_post = file_path.clone();
        let fp_put = file_path.clone();
        let fp_delete = file_path.clone();

        let rp_get = route_path.clone();
        let rp_post = route_path.clone();
        let rp_put = route_path.clone();
        let rp_delete = route_path.clone();

        app = app.route(
            &route_path.clone(),
            get(move |headers, query, body| {
                let file_path = fp_get.clone();
                let route_path = rp_get.clone();
                async move {
                    handle_api_route(headers, Method::GET, query, body, file_path, route_path).await
                }
            })
            .post(move |headers, query, body| {
                let file_path = fp_post.clone();
                let route_path = rp_post.clone();
                async move {
                    handle_api_route(headers, Method::POST, query, body, file_path, route_path)
                        .await
                }
            })
            .put(move |headers, query, body| {
                let file_path = fp_put.clone();
                let route_path = rp_put.clone();
                async move {
                    handle_api_route(headers, Method::PUT, query, body, file_path, route_path).await
                }
            })
            .delete(move |headers, query, body| {
                let file_path = fp_delete.clone();
                let route_path = rp_delete.clone();
                async move {
                    handle_api_route(headers, Method::DELETE, query, body, file_path, route_path)
                        .await
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
        Ok(response) => (
            StatusCode::from_u16(response.status).unwrap_or(StatusCode::OK),
            serde_json::to_string(&response.body).unwrap(),
        ),
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
