use tokio::fs::read_to_string;

use metacall::load;

pub async fn rust_runtime(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let rs_code = read_to_string(path).await?;

    let code = format!(
        r#"
          {}
"#,
        rs_code
    );

    // Load the RustJavaScript  code
    if let Err(e) = load::from_memory(load::Tag::Rust, &code, None) {
        return Err(Box::new(std::io::Error::other(format!(
            "MetaCall load error: {:?}",
            e
        ))));
    }

    Ok(String::from(""))
}
