use axum::Json;
use crate::GenericResponse;

pub async fn health() -> Json<GenericResponse> {
    let response = GenericResponse {
        message: "OK".to_string(),
    };
    Json(response)
}
