mod models;
mod routes;
mod utils;

use crate::routes::health::health;
use crate::routes::tenant::{add_tenant, get_tenants};
use axum::middleware;
use axum::routing::post;
use axum::{Router, routing::get};
use sqlx::sqlite::SqlitePoolOptions;
use utils::middleware::auth_validator;

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
        .route("/api/v1/tenants", post(add_tenant))
        .route("/api/v1/tenants", get(get_tenants))
        .with_state(pool);

    let app = Router::new()
        .route("/health", get(health))
        .merge(db_routes)
        .layer(middleware::from_fn(auth_validator));
    println!("Running on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
