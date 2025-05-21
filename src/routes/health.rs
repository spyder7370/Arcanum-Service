use crate::{models::json_response::GenericResponse, utils::response_utils::generate_response};
use axum::Json;

pub async fn health() -> Json<GenericResponse> {
    generate_response("OK", None).await
}
