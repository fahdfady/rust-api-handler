use tokio::fs::read_to_string;

use metacall::load::{self, Handle};

pub struct RsResponse {
    pub status: u16,
    pub body: String,
}

pub async fn execute_rust_file(path: &str) -> Result<RsResponse, Box<dyn std::error::Error>> {
    let rs_code = read_to_string(path).await?;

    let code = format!(
        r#"
          {}
"#,
        rs_code
    );

    let mut handle = Handle::new();

    // Load the RustJavaScript  code
    if let Err(e) = load::from_memory(load::Tag::Rust, &code, Some(&mut handle)) {
        return Err(Box::new(std::io::Error::other(format!(
            "MetaCall load error: {:?}",
            e
        ))));
    }

    Ok(RsResponse {
        status: 200,
        body: "".to_string(),
    })
}
