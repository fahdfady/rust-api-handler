use metacall::load::Handle;

use crate::{
    Lang, load_code,
    modules::{ApiRequest, ApiResponse},
};

pub async fn execute_js_file(
    path: &str,
    lang: Lang,
    request: ApiRequest,
) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    load_code(path, lang).await?;
    let mut handle = Handle::new();
    let request_json = serde_json::to_string(&request)?;

    // Call the handler function
    let result_str = metacall::metacall::<String>("GET", [request_json])
        .map_err(|e| format!("Failed to call handler: {:?}", e))?;
    print!("result : {result_str}");
    let response: ApiResponse =
        serde_json::from_str(&result_str).expect("couldn't change result to JsResponse");
    Ok(response)
}
