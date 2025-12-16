pub mod modules;

use axum::{
    Router,
    extract::Query,
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use metacall::load::{self, Handle, Tag};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::{
    fs::read_to_string,
    sync::{mpsc, oneshot},
};

use crate::modules::{ApiRequest, ApiResponse};

pub struct Route {
    pub path: String,
    pub code: String,
    pub lang: Tag,
}

pub struct Routes {
    pub routes: Vec<Route>,
    pub runtime: Arc<MetaCallRuntime>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            runtime: Arc::new(MetaCallRuntime::new()),
        }
    }

    pub async fn load_script(
        &mut self,
        file_path: &str,
        lang: Tag,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let code = read_to_string(file_path).await?;

        // Send load command to MetaCall runtime
        self.runtime
            .load_script(file_path.to_string(), code.clone(), lang)
            .await?;

        self.routes.push(Route {
            path: file_path.to_string(),
            code,
            lang,
        });

        Ok(())
    }

    pub async fn call_handler(
        &self,
        file_path: &str,
        method: &str,
        request: ApiRequest,
    ) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let request_json = serde_json::to_string(&request)?;

        // Call function through the runtime
        let result = self
            .runtime
            .call_function(
                file_path.to_string(),
                method.to_string(),
                vec![request_json],
            )
            .await?;

        let response: ApiResponse = serde_json::from_str(&result)?;
        Ok(response)
    }
}

impl Default for Routes {
    fn default() -> Self {
        Self::new()
    }
}

pub enum MetaCallCommand {
    LoadScript {
        path: String,
        code: String,
        lang: Tag,
        response: oneshot::Sender<Result<(), String>>,
    },
    CallFunction {
        script_path: String,
        function_name: String,
        args: Vec<String>,
        response: oneshot::Sender<Result<String, String>>,
    },
}

pub struct MetaCallRuntime {
    sender: mpsc::UnboundedSender<MetaCallCommand>,
}

impl MetaCallRuntime {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<MetaCallCommand>();

        std::thread::spawn(move || {
            let _metacall = metacall::initialize().expect("Failed to initialize metacall");
            // script path, Handle
            let mut handles: HashMap<String, Handle> = HashMap::new();

            while let Some(cmd) = rx.blocking_recv() {
                match cmd {
                    MetaCallCommand::LoadScript {
                        path,
                        code,
                        lang,
                        response,
                    } => {
                        let result = Self::load(&mut handles, &path, &code, lang);

                        response.send(result).unwrap();
                    }

                    MetaCallCommand::CallFunction {
                        script_path,
                        function_name,
                        args,
                        response,
                    } => {
                        let result = Self::call_function_blocking(
                            &mut handles,
                            &script_path,
                            &function_name,
                            args,
                        );

                        response.send(result).unwrap();
                    }
                }
            }
        });
        Self { sender: tx }
    }

    fn load(
        handles: &mut HashMap<String, Handle>,
        path: &str,
        code: &str,
        lang: Tag,
    ) -> Result<(), String> {
        let mut handle = Handle::new();

        load::from_memory(lang, code, Some(&mut handle))
            .map_err(|e| format!("Couldn't load from memory {:?}", e))?;

        handles.insert(path.to_string(), handle);
        Ok(())
    }

    fn call_function_blocking(
        handles: &mut HashMap<String, Handle>,
        script_path: &str,
        function_name: &str,
        args: Vec<String>,
    ) -> Result<String, String> {
        // Get the handle for this script
        let handle = handles
            .get_mut(script_path)
            .ok_or_else(|| format!("Script not loaded: {}", script_path))?;

        // Call the function
        println!(
            "Calling function '{}' in script '{}' with args: {:?}",
            function_name, script_path, args
        );
        let result = metacall::metacall_handle::<String>(handle, function_name, args)
            .map_err(|e| format!("Failed to call {}: {:?}", function_name, e))?;

        Ok(result)
    }

    pub async fn load_script(&self, path: String, code: String, lang: Tag) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();

        self.sender
            .send(MetaCallCommand::LoadScript {
                path,
                code,
                lang,
                response: tx,
            })
            .map_err(|_| "MetaCall runtime disconnected".to_string())?;

        rx.await
            .map_err(|_| "Response channel closed".to_string())?
    }

    // Public async API for calling functions
    pub async fn call_function(
        &self,
        script_path: String,
        function_name: String,
        args: Vec<String>,
    ) -> Result<String, String> {
        let (tx, rx) = oneshot::channel();

        self.sender
            .send(MetaCallCommand::CallFunction {
                script_path,
                function_name,
                args,
                response: tx,
            })
            .map_err(|_| "MetaCall runtime disconnected".to_string())?;

        rx.await
            .map_err(|_| "Response channel closed".to_string())?
    }
}

#[tokio::main]
async fn main() {
    println!("(´｡• ᵕ •｡`) Starting server on http://localhost:3000");
    let mut app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let mut routes = Routes::new();

    let api_files = scan_api_dir("api");

    println!("found {} route(s)", api_files.len());

    for (route_path, file_path, lang) in &api_files {
        println!("  - {}", route_path);

        if let Err(e) = routes.load_script(&file_path, *lang).await {
            eprintln!("Couldn't load route {}: {:?}", file_path, e);
            continue;
        };
    }

    let routes = Arc::new(routes);

    for (route_path, file_path, lang) in api_files {
        let routes = Arc::clone(&routes);
        app = app.route(
            &route_path.clone(),
            get({
                let routes = Arc::clone(&routes);
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move |headers, query, body| {
                    handle_api_route(
                        routes,
                        headers,
                        Method::GET,
                        query,
                        body,
                        file_path,
                        route_path,
                    )
                    .await
                }
            })
            .post({
                let routes = Arc::clone(&routes);
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move |headers, query, body| {
                    handle_api_route(
                        routes,
                        headers,
                        Method::POST,
                        query,
                        body,
                        file_path,
                        route_path,
                    )
                    .await
                }
            })
            .put({
                let routes = Arc::clone(&routes);
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move |headers, query, body| {
                    handle_api_route(
                        routes,
                        headers,
                        Method::PUT,
                        query,
                        body,
                        file_path,
                        route_path,
                    )
                    .await
                }
            })
            .delete(move |headers, query, body| {
                let routes = Arc::clone(&routes);
                let file_path = file_path.clone();
                let route_path = route_path.clone();
                async move {
                    handle_api_route(
                        routes,
                        headers,
                        Method::DELETE,
                        query,
                        body,
                        file_path,
                        route_path,
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

async fn handle_api_route(
    routes: Arc<Routes>,
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

    let request = ApiRequest {
        url: route_path,
        headers: headers_map,
        method: method.to_string(),
        query,
        body: if body.is_empty() { None } else { Some(body) },
        params: HashMap::new(),
    };

    match routes
        .call_handler(&file_path, method.as_ref(), request)
        .await
    {
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

/// Scan the api directory and return tuple of route names & paths to files & Language
fn scan_api_dir(dir: &str) -> Vec<(String, String, Tag)> {
    let mut routes = Vec::new();
    let base_path = PathBuf::from(dir);

    if !base_path.exists() {
        eprint!("Directory {base_path:?} not found");
        return routes;
    }

    scan_api_dir_recursive(&base_path, &base_path, &mut routes);
    routes
}

/// Recursively scan directories for API files
fn scan_api_dir_recursive(
    base_path: &PathBuf,
    current_path: &PathBuf,
    routes: &mut Vec<(String, String, Tag)>,
) {
    if let Ok(entries) = current_path.read_dir() {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                // Recursively scan subdirectories
                scan_api_dir_recursive(base_path, &path, routes);
            } else if path.is_file() {
                // Check if file has a supported extension
                let lang = match path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
                    Some("js") => Some(Tag::NodeJS),
                    Some("ts") => Some(Tag::TypeScript),
                    Some("py") => Some(Tag::Python),
                    // Some("rb") => Some(Tag::Ruby),
                    _ => None,
                };

                if let Some(lang) = lang {
                    // Build route path from directory structure
                    // Remove base_path and file extension to get the route
                    if let Ok(relative_path) = path.strip_prefix(base_path) {
                        let route_parts: Vec<&str> =
                            relative_path.iter().filter_map(|s| s.to_str()).collect();

                        // Remove the file extension from the last part
                        let mut route_path = String::from("/api");
                        for (i, part) in route_parts.iter().enumerate() {
                            if i == route_parts.len() - 1 {
                                // Last part is the filename, remove extension
                                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                    route_path.push('/');
                                    route_path.push_str(stem);
                                }
                            } else {
                                // Directory name
                                route_path.push('/');
                                route_path.push_str(part);
                            }
                        }

                        let file_path = path.to_string_lossy().to_string();
                        routes.push((route_path, file_path, lang));
                    }
                }
            }
        }
    }
}
