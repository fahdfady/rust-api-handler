use std::fmt::format;

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
    let method = &request.method;
    let request_json = serde_json::to_string(&request)?;

    let result_str = tokio::task::spawn_blocking(move || -> Result<String, String> {
        match metacall::metacall::<String>("GET", [request_json.clone()]) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("MetaCall Error: {:?}", e)),
        }
    })
    .await??;

    print!("result : {result_str}");
    let response: ApiResponse =
        serde_json::from_str(&result_str).expect("couldn't change result to JsResponse");
    Ok(response)
}
