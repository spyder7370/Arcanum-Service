use axum::http::{Method, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use super::response_utils::generate_error;

pub async fn auth_validator(
    request: Request<axum::body::Body>,
    next: Next<axum::body::Body>,
) -> Response {
    if request.method() != Method::GET {
        match request.headers().get("Authorization") {
            Some(value) if value == "pleaseletmein" => next.run(request).await,
            _ => {
                (generate_error(Some("UNAUTHORIZED".to_string()), Some(401)).await).into_response()
            }
        }
    } else {
        next.run(request).await
    }
}
