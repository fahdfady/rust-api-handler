use std::fs::read_to_string;

use deno_core::JsRuntime;

pub fn execute_js_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut runtime = JsRuntime::new(Default::default());

    let js_code = read_to_string(path)?;

    let code = format!(
        r#"
        {}
        
        // Call the handler function
        const result = handler();
        result;
        "#,
        js_code
    );

    let result = runtime.execute_script("<anon>", code)?;

    let mut scope = runtime.handle_scope();
    let local = deno_core::v8::Local::new(&mut scope, result);

    let result_str = local.to_rust_string_lossy(&mut scope);
    Ok(result_str)
}
