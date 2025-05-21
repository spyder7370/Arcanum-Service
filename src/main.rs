mod models;
mod routes;

use crate::routes::health::health;
use crate::routes::tenant::{add_tenant, get_tenants};
use axum::routing::post;
use axum::{Json, Router, routing::get};
use routes::health;
use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .connect("sqlite:Arcanum.sqlite")
        .await
        .unwrap();

    let db_routes = Router::new()
        .route("/api/v1/tenant", post(add_tenant))
        .route("/api/v1/tenant", get(get_tenants))
        .with_state(pool);

    let app = Router::new().route("/health", get(health)).merge(db_routes);
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
