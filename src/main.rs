use axum::{Json, Router, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let doc_url = "swagger/openapi.json";
    let app = Router::new().route("/health", get(health));
    println!("Running on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct GenericResponse {
    message: String,
}

async fn health() -> Json<GenericResponse> {
    let response = GenericResponse {
        message: "OK".to_string(),
    };
    Json(response)
}
