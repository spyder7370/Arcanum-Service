use axum::Json;
use serde::Serialize;

use crate::models::json_response::{GenericError, GenericResponse};

pub async fn generate_error(message: Option<String>, code: Option<i32>) -> Json<GenericResponse> {
    let error = GenericError {
        message: Some(
            message
                .unwrap_or("Something went wrong".to_string())
                .to_string(),
        ),
    };
    let response = GenericResponse {
        data: None,
        code: Some(code.unwrap_or(505)),
        error: error,
    };
    Json(response)
}

pub async fn generate_response<T: Serialize>(data: T, code: Option<i32>) -> Json<GenericResponse> {
    let error = GenericError { message: None };
    let response = GenericResponse {
        data: Some(serde_json::to_value(data).unwrap()),
        code: Some(code.unwrap_or(200)),
        error,
    };
    Json(response)
}
