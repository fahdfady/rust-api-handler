use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

use deno_core::JsRuntime;

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
    let js_code = read_to_string(path).await?;

    let mut runtime = JsRuntime::new(Default::default());

    let request_json = serde_json::to_string(&request)?;

    let code = format!(
        r#"
        {}

        let request = {};
        if (!request) throw new Error("No request found");
        let method = request["method"];

        // Call the appropiate handler function
        const handlerFn = globalThis[method];

        if (typeof handlerFn !== 'function') {{
            // Method not supported
            JSON.stringify({{
                status: 405,
                body: JSON.stringify({{ error: 'Method ' + request.method + ' not allowed' }})
            }});
        }} else {{
            // Call the handler
            const result = handlerFn(request);
            result;
        }}

        "#,
        js_code, request_json
    );

    let result = runtime
        .execute_script("<anon>", code)
        .expect("couldn't execute code at runtime");

    let mut scope = runtime.handle_scope();
    let local = deno_core::v8::Local::new(&mut scope, result);
    let result_str = local.to_rust_string_lossy(&mut scope);

    let response: JsResponse = serde_json::from_str::<JsResponse>(&result_str)?;
    Ok(response)
}
