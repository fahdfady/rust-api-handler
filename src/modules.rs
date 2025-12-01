use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type Headers = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiRequest {
    pub url: String,
    pub headers: Headers,
    pub method: String,
    pub body: Option<String>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    pub status: u16,
    pub headers: Headers,
    pub body: serde_json::Value,
}

#[derive(Clone, Copy, Debug)]
pub enum Lang {
    Rust,
    JavaScript,
    TypeScript,
    NodeJS,
}
