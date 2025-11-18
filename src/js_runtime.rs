use metacall::load;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::fs::read_to_string;

// Wrapper to make Handle Send + Sync (we ensure thread safety with Mutex)
struct SendHandle(load::Handle);
unsafe impl Send for SendHandle {}
unsafe impl Sync for SendHandle {}

// Global handle that persists across requests
static METACALL_HANDLE: Lazy<Mutex<SendHandle>> =
    Lazy::new(|| Mutex::new(SendHandle(load::Handle::new())));

#[derive(Serialize, Deserialize, Debug)]
pub struct JsRequest {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub method: String,
    pub body: Option<String>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsResponse {
    pub status: u16,
    pub body: serde_json::Value,
}

pub async fn execute_js_file(
    path: &str,
    request: JsRequest,
) -> Result<JsResponse, Box<dyn std::error::Error>> {
    let _metacall = metacall::initialize().unwrap();

    let js_code = read_to_string(path).await?;
    let request_json = serde_json::to_string(&request)?;

    // Generate a unique function name to avoid conflicts
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let code = format!(
        r#"
const request_{} = {};
const method_{} = request_{}.method;

{}

function handler_{}() {{
    const handlerFn = globalThis[method_{}];
    if (typeof handlerFn !== 'function') {{
        return JSON.stringify({{
            status: 405,
            body: {{ error: 'Method ' + method_{} + ' not allowed' }}
        }});
    }}
    const result = handlerFn(request_{});
    return JSON.stringify(result);
}}

module.exports = {{ handler_{}: handler_{} }}; "#,
        unique_id,
        request_json,
        unique_id,
        unique_id,
        js_code,
        unique_id,
        unique_id,
        unique_id,
        unique_id,
        unique_id,
        unique_id
    );

    // Lock the global handle (blocks if another thread is using it)
    let mut handle = METACALL_HANDLE.lock().unwrap();

    // Load the JavaScript code
    if let Err(e) = load::from_memory(load::Tag::NodeJS, &code, Some(&mut handle.0)) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("MetaCall load error: {:?}", e),
        )));
    }

    // Call the handler function
    let result_str =
        metacall::metacall::<String>(&format!("handler_{}", unique_id), Vec::<i32>::new())
            .map_err(|e| format!("Failed to call handler: {:?}", e))?;

    let response: JsResponse = serde_json::from_str(&result_str)?;

    Ok(response)
}
