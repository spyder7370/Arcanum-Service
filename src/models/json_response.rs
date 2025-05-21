use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GenericError {
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GenericResponse {
    pub data: Option<serde_json::Value>,
    pub code: Option<i32>,
    pub error: GenericError,
}
