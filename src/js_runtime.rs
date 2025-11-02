use metacall::load;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs::read_to_string;

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

    // Create a module that exports the handler function
    let code = format!(
        r#"
const request = {};
const method = request.method;

{}

function handler() {{
    const handlerFn = globalThis[method];
    if (typeof handlerFn !== 'function') {{
        return JSON.stringify({{
            status: 405,
            body: {{ error: 'Method ' + method + ' not allowed' }}
        }});
    }}
    const result = handlerFn(request);
    return JSON.stringify(result);
}}

module.exports = {{ handler }};
"#,
        request_json, js_code
    );

    // Load and execute the JavaScript code using MetaCall
    if let Err(e) = load::from_memory(load::Tag::NodeJS, &code, None) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("MetaCall load error: {:?}", e),
        )));
    }

    // Call the handler function with no arguments
    let result_str = metacall::metacall::<String>("handler", Vec::<i32>::new())
        .map_err(|e| format!("Failed to call handler: {:?}", e))?;

    let response: JsResponse = serde_json::from_str(&result_str)?;

    Ok(response)
}
