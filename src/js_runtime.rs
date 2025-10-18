use std::collections::HashMap;

use tokio::fs::read_to_string;

use deno_core::JsRuntime;

pub struct JsRequest {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub method: String,
    pub body: Option<String>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

struct JsResponse {
    headers: HashMap<String, String>,
    body: Option<String>,
    status: Option<String>,
}

pub async fn execute_js_file(path: &str, request: JsRequest) -> Result<String, Box<dyn std::error::Error>> {
    let js_code = read_to_string(path).await?;

    let mut runtime = JsRuntime::new(Default::default());

    let code = format!(
        r#"
        console.log("{}");
        console.log("hey");

        {}
        
        // Call the GET handler function
        const result = GET();
        result;
        "#,
        request.method,
        js_code
    );

    // println!("{code}");

    let result = runtime.execute_script("<anon>", code)?;

    let mut scope = runtime.handle_scope();
    let local = deno_core::v8::Local::new(&mut scope, result);

    let result_str = local.to_rust_string_lossy(&mut scope);
    Ok(result_str)
}
