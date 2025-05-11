use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Clone,Deserialize)]
pub struct JsonResultResponse<T>{
    pub status: String,
    pub result: Option<T>,
    pub error: Option<String>
}