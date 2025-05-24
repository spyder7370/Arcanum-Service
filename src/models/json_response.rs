use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Clone)]
pub struct JsonResultResponse<T>{
    pub status: String,
    pub result: Option<T>,
    pub error: Option<String>,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
}

impl<T: Serialize> JsonResultResponse<T> {
    pub fn success(result: T) -> Self {
        JsonResultResponse {
            status: String::from("SUCCESS"),
            result: Some(result),
            error: None,
            status_code: StatusCode::OK,
        }
    }
    pub fn error(error: String,status_code: StatusCode) -> Self {
        JsonResultResponse {
            status: String::from("ERROR"),
            result: None,
            error: Some(error),
            status_code,
        }
    }
}

impl <T: Serialize> IntoResponse for JsonResultResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, serde_json::to_string(&self).unwrap()).into_response()
    }
}


// impl<T: Serialize> JsonResultResponse<T> {
//     pub const STATUS_SUCCESS: &'static str = "SUCCESS";
//     pub const STATUS_ERROR: &'static str = "ERROR";
// }