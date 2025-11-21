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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsResponse {
    pub status: u16,
    pub body: serde_json::Value,
}

pub async fn execute_js_file(
    path: &str,
    request: JsRequest,
) -> Result<JsResponse, Box<dyn std::error::Error>> {
    let js_code = read_to_string(path).await?;
    let request_json = serde_json::to_string(&request)?;

    // Generate a unique function name to avoid conflicts
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let code = format!(
        r#"
             {}
             
             const handlers = {{ GET, POST, PUT, DELETE }};
            
             const request = {};
             const method = request.method;

             function handler() {{
                 console.log(method);
                 const handlerFn = handlers[method];
                 console.log(handlerFn);
                 if (typeof handlerFn !== 'function') {{
                     return JSON.stringify({{
                         status: 405,
                         body: {{ error: 'Method ' + method + ' not allowed' }}
                     }});
                 }}
                 const result = handlerFn(request);
                 return result;
             }}
             
             module.exports = {{ handler }}; "#,
        js_code, request_json,
    );

    println!("{code}");

    // Load the JavaScript code
    if let Err(e) = load::from_memory(load::Tag::NodeJS, &code, None) {
        return Err(Box::new(std::io::Error::other(format!(
            "MetaCall load error: {:?}",
            e
        ))));
    }

    // Call the handler function
    let result_str = metacall::metacall::<String>(&format!("handler"), Vec::<i32>::new())
        .map_err(|e| format!("Failed to call handler: {:?}", e))?;

    println!("result: {result_str}");
    let response: JsResponse =
        serde_json::from_str(&result_str).expect("couldn't change result to JsResponse");

    println!("response: {:?}", response.clone());

    Ok(response)
}
