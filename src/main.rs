mod runtimes;

use axum::{
    Router,
    extract::Query,
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::{collections::HashMap, fmt, path::PathBuf};

use crate::runtimes::{
    js_runtime::{JsRequest, execute_js_file},
    rs_runtime::execute_rust_file,
};

#[derive(Clone, Copy, Debug)]
pub enum Lang {
    Rust,
    JavaScript,
    TypeScript,
    NodeJS,
}

#[tokio::main]
async fn main() {
    println!("(´｡• ᵕ •｡`) Starting server on http://localhost:3000");
    let mut app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let _metacall = metacall::initialize().unwrap();

    let routes = scan_api_dir("api");

    println!("found {} route(s)", routes.len());

    // rust_runtime("api/greet.rs");

    for (route_path, file_path, lang) in routes {
        println!("  - {}", route_path);

        app = app.route(
            &route_path.clone(),
            get({
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move |headers, query, body| {
                    handle_api_route(
                        headers,
                        Method::GET,
                        query,
                        body,
                        file_path,
                        route_path,
                        lang,
                    )
                    .await
                }
            })
            .post({
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move |headers, query, body| {
                    handle_api_route(
                        headers,
                        Method::POST,
                        query,
                        body,
                        file_path,
                        route_path,
                        lang,
                    )
                    .await
                }
            })
            .put({
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move |headers, query, body| {
                    handle_api_route(
                        headers,
                        Method::PUT,
                        query,
                        body,
                        file_path,
                        route_path,
                        lang,
                    )
                    .await
                }
            })
            .delete(move |headers, query, body| {
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move {
                    handle_api_route(
                        headers,
                        Method::DELETE,
                        query,
                        body,
                        file_path,
                        route_path,
                        lang,
                    )
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
    lang: Lang,
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

    match lang {
        Lang::NodeJS => match execute_js_file(&file_path, lang, js_request).await {
            Ok(response) => (
                StatusCode::from_u16(response.status).unwrap_or(StatusCode::OK),
                serde_json::to_string(&response.body).unwrap(),
            ),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"error\": \"{}\"}}", error),
            ),
        },

        Lang::TypeScript => match execute_js_file(&file_path, lang, js_request).await {
            Ok(response) => (
                StatusCode::from_u16(response.status).unwrap_or(StatusCode::OK),
                serde_json::to_string(&response.body).unwrap(),
            ),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"error\": \"{}\"}}", error),
            ),
        },
        Lang::Rust => match execute_rust_file(&file_path).await {
            Ok(response) => (
                StatusCode::from_u16(response.status).unwrap_or(StatusCode::OK),
                serde_json::to_string(&response.body).unwrap(),
            ),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"error\": \"{}\"}}", error),
            ),
        },
        _ => todo!(),
    }
}

/// Scan the api directory and return tuple of route names & paths to files & Language
fn scan_api_dir(dir: &str) -> Vec<(String, String, Lang)> {
    let mut routes = Vec::new();

    let path = PathBuf::from(dir);

    if !path.exists() {
        eprint!("Directory {path:?} not found")
    }

    if let Ok(entries) = path.read_dir() {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let lang = match path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
                    Some("js") => Some(Lang::NodeJS),
                    Some("rs") => Some(Lang::Rust),
                    Some("ts") => Some(Lang::TypeScript),
                    _ => None,
                };
                if let Some(lang) = lang
                    && let Some(file_name) = path.file_stem()
                {
                    let route = format!("/api/{}", file_name.to_str().unwrap());

                    let file_path = path.to_string_lossy().to_string();
                    routes.push((route, file_path, lang));
                }
            }
        }
    }

    routes
}
